use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeSet, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

// Positions INSIDE a corpus, so they hold for any repository that adopts the
// layout. Resolving the review assets relative to the corpus root rather than the
// repository root is what keeps tool and corpus co-located: `review` reads both
// from the filesystem at runtime, so a corpus that moves takes them with it.
const SEMANTIC_REVIEW_SUBDIR: &str = "15-evaluation/semantic-review";
const REVIEW_PROMPT_ASSET: &str = "16-enforcement/review-prompt.md";
const REVIEW_SCHEMA_ASSET: &str = "16-enforcement/review-result.schema.json";

#[derive(Parser, Debug)]
pub struct VrsCli {
    #[command(subcommand)]
    pub cmd: VrsCmd,
}

#[derive(Subcommand, Debug)]
pub enum VrsCmd {
    /// Run deterministic VRS checks.
    Check(CheckArgs),
    /// Emit the derived VRS graph subset.
    Graph(GraphArgs),
    /// Run semantic VRS review through the Coding Agent Invocation Contract.
    Review(ReviewArgs),
    /// Grade semantic review against evaluation-fixture minimum assertions.
    ReviewFixtures(ReviewFixturesArgs),
}

#[derive(Parser, Debug)]
pub struct CheckArgs {
    /// VRS root to check.
    pub root: Option<PathBuf>,

    /// Rule profile to run.
    #[arg(long, value_enum, default_value_t = Profile::Local)]
    pub profile: Profile,

    /// Emit machine-readable diagnostics JSON.
    #[arg(long)]
    pub json: bool,

    /// Treat warnings as errors.
    #[arg(long)]
    pub warnings_as_errors: bool,
}

#[derive(Parser, Debug)]
pub struct GraphArgs {
    /// VRS root to graph.
    pub root: Option<PathBuf>,

    /// Emit machine-readable graph JSON.
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct ReviewArgs {
    /// VRS root to review.
    pub root: Option<PathBuf>,

    /// Rule profile to use for the deterministic diagnostics packet.
    #[arg(long, value_enum, default_value_t = Profile::Local)]
    pub profile: Profile,

    /// Coding Agent Invocation Contract executable.
    #[arg(long, env = "CODING_AGENT", default_value = "coding-agent")]
    pub coding_agent: PathBuf,

    /// CAIC backend id.
    #[arg(long)]
    pub backend: Option<String>,

    /// Whole-run timeout in seconds.
    #[arg(long)]
    pub timeout_seconds: Option<u64>,

    /// Write the review envelope to a file instead of stdout.
    #[arg(long)]
    pub report: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct ReviewFixturesArgs {
    /// Semantic-review fixture root.
    pub root: Option<PathBuf>,

    /// Fixture id to grade. Repeatable; defaults to every fixture under the root.
    #[arg(long = "fixture")]
    pub fixtures: Vec<String>,

    /// Rule profile to use for the deterministic diagnostics packet.
    #[arg(long, value_enum, default_value_t = Profile::Local)]
    pub profile: Profile,

    /// Coding Agent Invocation Contract executable.
    #[arg(long, env = "CODING_AGENT", default_value = "coding-agent")]
    pub coding_agent: PathBuf,

    /// CAIC backend id.
    #[arg(long)]
    pub backend: Option<String>,

    /// Per-fixture review timeout in seconds.
    #[arg(long)]
    pub timeout_seconds: Option<u64>,

    /// Emit machine-readable grading JSON.
    #[arg(long)]
    pub json: bool,

    /// Write the grading report to a file instead of stdout.
    #[arg(long)]
    pub report: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Profile {
    Local,
    Strict,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
    pub schema_version: &'static str,
    pub kind: &'static str,
    pub severity: Severity,
    pub gate: &'static str,
    pub artifact: String,
    pub owner: String,
    pub rule: &'static str,
    pub evidence: String,
    pub suggested_fix: String,
}

#[derive(Debug, Serialize)]
pub struct CheckReport {
    pub schema_version: &'static str,
    pub root: String,
    pub profile: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize)]
pub struct GraphReport {
    pub schema_version: &'static str,
    pub root: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub path: String,
    pub status: String,
    pub refs: Vec<String>,
    pub refines: Vec<String>,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub path: String,
    pub evidence: String,
}

/// Where to look when the caller gave no path.
///
/// Which directory holds a corpus is the embedding repository's layout policy, not
/// something a reusable checker can know: `intent` defaults to the directory it is
/// run in, while `axe vrs` supplies dotfiles' own `context/vrs`. Baking one repo's
/// layout in here is what made the tool unusable anywhere else, and a wrong default
/// is invisible — checking a path that holds no VRS artifacts exits 0.
pub struct Defaults {
    corpus_root: PathBuf,
}

impl Defaults {
    pub fn corpus_root(root: impl Into<PathBuf>) -> Self {
        Self {
            corpus_root: root.into(),
        }
    }

    fn root_or_default(&self, arg: Option<PathBuf>) -> PathBuf {
        arg.unwrap_or_else(|| self.corpus_root.clone())
    }

    // Fixtures live at a fixed position INSIDE the corpus, so one caller-supplied
    // corpus root determines both defaults and they cannot drift apart.
    fn fixtures_or_default(&self, arg: Option<PathBuf>) -> PathBuf {
        arg.unwrap_or_else(|| self.corpus_root.join(SEMANTIC_REVIEW_SUBDIR))
    }
}

impl Default for Defaults {
    fn default() -> Self {
        Self::corpus_root(".")
    }
}

pub fn run(cli: VrsCli) -> ExitCode {
    run_with(cli, &Defaults::default())
}

/// `run` with the caller's layout policy. `axe vrs` uses this to keep its own
/// `context/vrs` default, so extracting this crate did not change its behavior.
pub fn run_with(cli: VrsCli, defaults: &Defaults) -> ExitCode {
    match cli.cmd {
        VrsCmd::Check(args) => run_check(args, defaults),
        VrsCmd::Graph(args) => run_graph(args, defaults),
        VrsCmd::Review(args) => run_review(args, defaults),
        VrsCmd::ReviewFixtures(args) => run_review_fixtures(args, defaults),
    }
}

fn run_check(args: CheckArgs, defaults: &Defaults) -> ExitCode {
    let root = defaults.root_or_default(args.root);
    let report = match check_root(&root, args.profile) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("axe vrs check: {error}");
            return ExitCode::from(2);
        }
    };
    let has_errors = report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == Severity::Error
            || (args.warnings_as_errors && diagnostic.severity == Severity::Warning)
    });

    if args.json {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => println!("{json}"),
            Err(error) => {
                eprintln!("axe vrs check: failed to render json: {error}");
                return ExitCode::from(2);
            }
        }
    } else if report.diagnostics.is_empty() {
        println!("axe vrs check: ok");
    } else {
        for diagnostic in &report.diagnostics {
            println!(
                "{} {} {}: {}",
                severity_label(&diagnostic.severity),
                diagnostic.rule,
                diagnostic.artifact,
                diagnostic.evidence
            );
        }
    }

    if has_errors {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn run_graph(args: GraphArgs, defaults: &Defaults) -> ExitCode {
    let root = defaults.root_or_default(args.root);
    let report = match graph_root(&root) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("axe vrs graph: {error}");
            return ExitCode::from(2);
        }
    };

    if args.json {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => println!("{json}"),
            Err(error) => {
                eprintln!("axe vrs graph: failed to render json: {error}");
                return ExitCode::from(2);
            }
        }
    } else {
        println!(
            "axe vrs graph: {} nodes, {} edges",
            report.nodes.len(),
            report.edges.len()
        );
    }

    ExitCode::SUCCESS
}

fn run_review(args: ReviewArgs, defaults: &Defaults) -> ExitCode {
    let args_root = defaults.root_or_default(args.root);
    if let Some(indicator) = automated_context_indicator() {
        eprintln!("axe vrs review: refusing semantic review in automated context ({indicator})");
        return ExitCode::from(2);
    }

    if let Err(error) = preflight_review_backend(&args.coding_agent, args.backend.as_deref()) {
        eprintln!("axe vrs review: {error}");
        return ExitCode::from(2);
    }

    let root = match fs::canonicalize(&args_root) {
        Ok(root) => root,
        Err(error) => {
            eprintln!(
                "axe vrs review: invalid root {}: {error}",
                args_root.display()
            );
            return ExitCode::from(2);
        }
    };
    if !root.is_dir() {
        eprintln!(
            "axe vrs review: root is not a directory: {}",
            root.display()
        );
        return ExitCode::from(2);
    }
    let workspace = review_workspace(&root);
    let prompt = match corpus_asset(&root, REVIEW_PROMPT_ASSET) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("axe vrs review: {error}");
            return ExitCode::from(2);
        }
    };
    let schema = match corpus_asset(&root, REVIEW_SCHEMA_ASSET) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("axe vrs review: {error}");
            return ExitCode::from(2);
        }
    };

    let invocation = ReviewInvocation {
        coding_agent: &args.coding_agent,
        backend: args.backend.as_deref(),
        timeout_seconds: args.timeout_seconds,
        root: &root,
        workspace: &workspace,
        prompt: &prompt,
        schema: &schema,
        profile: args.profile,
        final_output: args.report.as_deref(),
    };
    let mut plan = match prepare_review(&invocation) {
        Ok(plan) => plan,
        Err(error) => {
            eprintln!("axe vrs review: {}", error.message);
            return ExitCode::from(error.exit_code);
        }
    };

    let output = match plan.command.output() {
        Ok(output) => output,
        Err(error) => {
            eprintln!(
                "axe vrs review: failed to start CAIC executable {}: {error}",
                args.coding_agent.display()
            );
            return ExitCode::from(2);
        }
    };
    let _ = io::stdout().write_all(&output.stdout);
    let _ = io::stderr().write_all(&output.stderr);
    match output.status.code() {
        Some(code) => ExitCode::from(code as u8),
        None => ExitCode::from(3),
    }
}

struct ReviewInvocation<'a> {
    coding_agent: &'a Path,
    backend: Option<&'a str>,
    timeout_seconds: Option<u64>,
    /// VRS tree to review.
    root: &'a Path,
    /// Directory the coding agent runs in. Finding artifact paths are reported
    /// relative to it, so it decides what an artifact path in a review result means.
    workspace: &'a Path,
    prompt: &'a Path,
    schema: &'a Path,
    profile: Profile,
    final_output: Option<&'a Path>,
}

struct ReviewPlan {
    command: Command,
    /// Holds the generated diagnostics packet alive until the command has run.
    _diagnostics: tempfile::TempDir,
}

struct ReviewSetupError {
    message: String,
    exit_code: u8,
}

impl ReviewSetupError {
    fn tool(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 2,
        }
    }
}

/// Build the CAIC invocation for one semantic review, including the deterministic
/// diagnostics packet. Shared by `axe vrs review` and `axe vrs review-fixtures` so
/// fixture grading exercises the same invocation the operator command uses.
fn prepare_review(invocation: &ReviewInvocation<'_>) -> Result<ReviewPlan, ReviewSetupError> {
    let target_files = match markdown_files(invocation.root) {
        Ok(files) if !files.is_empty() => files,
        Ok(_) => {
            return Err(ReviewSetupError {
                message: format!(
                    "no markdown VRS artifacts found under {}",
                    invocation.root.display()
                ),
                exit_code: 1,
            })
        }
        Err(error) => {
            return Err(ReviewSetupError::tool(format!(
                "failed to collect VRS artifacts: {error}"
            )))
        }
    };
    if let Some(outside) = target_files
        .iter()
        .find(|path| !path.starts_with(invocation.workspace))
    {
        return Err(ReviewSetupError::tool(format!(
            "target artifact escapes review workspace {}: {}",
            invocation.workspace.display(),
            outside.display()
        )));
    }

    let report = check_root(invocation.root, invocation.profile)
        .map_err(|error| ReviewSetupError::tool(format!("deterministic check failed: {error}")))?;
    let tempdir = tempfile::tempdir().map_err(|error| {
        ReviewSetupError::tool(format!("failed to create diagnostics packet: {error}"))
    })?;
    let diagnostics_path = tempdir.path().join("axe-vrs-check.json");
    let diagnostics_packet = serde_json::json!({
        "producer": "axe vrs check --json",
        "schema_version": report.schema_version,
        "root": report.root,
        "profile": report.profile,
        "diagnostics": report.diagnostics,
    });
    let rendered = serde_json::to_vec_pretty(&diagnostics_packet).map_err(|error| {
        ReviewSetupError::tool(format!("failed to render diagnostics packet: {error}"))
    })?;
    fs::write(&diagnostics_path, rendered).map_err(|error| {
        ReviewSetupError::tool(format!("failed to write diagnostics packet: {error}"))
    })?;

    let mut command = Command::new(invocation.coding_agent);
    command
        .arg("run")
        .arg("--cwd")
        .arg(invocation.workspace)
        .arg("--prompt-file")
        .arg(invocation.prompt)
        .arg("--mode")
        .arg("review")
        .arg("--permission")
        .arg("read-only")
        .arg("--approval")
        .arg("never")
        .arg("--config-policy")
        .arg("isolated")
        .arg("--network-policy")
        .arg("disabled")
        .arg("--output-format")
        .arg("json")
        .arg("--output-schema")
        .arg(invocation.schema)
        .arg("--context-file")
        .arg(format!(
            "generated-diagnostics:{}",
            diagnostics_path.display()
        ));
    if let Some(backend) = invocation.backend {
        command.arg("--backend").arg(backend);
    }
    if let Some(timeout_seconds) = invocation.timeout_seconds {
        command
            .arg("--timeout-seconds")
            .arg(timeout_seconds.to_string());
    }
    if let Some(final_output) = invocation.final_output {
        command.arg("--final-output").arg(final_output);
    }
    for path in target_files {
        command
            .arg("--context-file")
            .arg(format!("normative:{}", path.display()));
    }

    Ok(ReviewPlan {
        command,
        _diagnostics: tempdir,
    })
}

#[derive(Debug, Serialize)]
pub struct FixtureGradingReport {
    pub schema_version: &'static str,
    pub fixtures_root: String,
    pub backend: Option<String>,
    pub fixtures: Vec<FixtureGrade>,
    pub passed: usize,
    pub failed: usize,
    pub errored: usize,
    pub skipped: usize,
}

#[derive(Debug, Serialize)]
pub struct FixtureGrade {
    pub id: String,
    /// `passed`, `failed` (assertions unmet), `errored` (review did not run), or
    /// `skipped` (fixture declares no minimum assertions).
    pub status: &'static str,
    pub assertion_mode: Option<String>,
    pub matched: Vec<MinimumFinding>,
    pub missing: Vec<MinimumFinding>,
    pub reason: Option<String>,
}

/// The stable comparison contract from decision 0027: a review result must contain
/// a finding with this `rule`, `severity`, `artifact`, and `owner`. Summary,
/// evidence, and suggested-fix wording are deliberately not compared.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct MinimumFinding {
    pub rule: String,
    pub severity: String,
    pub artifact: String,
    pub owner: String,
}

fn run_review_fixtures(args: ReviewFixturesArgs, defaults: &Defaults) -> ExitCode {
    let args_root = defaults.fixtures_or_default(args.root.clone());
    if let Some(indicator) = automated_context_indicator() {
        eprintln!(
            "axe vrs review-fixtures: refusing fixture review in automated context ({indicator})"
        );
        return ExitCode::from(2);
    }

    if let Err(error) = preflight_review_backend(&args.coding_agent, args.backend.as_deref()) {
        eprintln!("axe vrs review-fixtures: {error}");
        return ExitCode::from(2);
    }

    let root = match fs::canonicalize(&args_root) {
        Ok(root) if root.is_dir() => root,
        Ok(root) => {
            eprintln!(
                "axe vrs review-fixtures: fixtures root is not a directory: {}",
                root.display()
            );
            return ExitCode::from(2);
        }
        Err(error) => {
            eprintln!(
                "axe vrs review-fixtures: invalid fixtures root {}: {error}",
                args_root.display()
            );
            return ExitCode::from(2);
        }
    };

    let selected = match select_fixtures(&root, &args.fixtures) {
        Ok(selected) => selected,
        Err(error) => {
            eprintln!("axe vrs review-fixtures: {error}");
            return ExitCode::from(2);
        }
    };

    // Decision 0024: eval runs materialize tracked fixtures into an isolated
    // temporary workspace instead of running against the tracked tree.
    let workspaces = match tempfile::tempdir() {
        Ok(workspaces) => workspaces,
        Err(error) => {
            eprintln!("axe vrs review-fixtures: failed to create eval workspace: {error}");
            return ExitCode::from(2);
        }
    };

    let mut grades = Vec::new();
    for fixture in &selected {
        let id = fixture
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .to_string();
        grades.push(grade_fixture(&args, fixture, &id, workspaces.path()));
    }

    let passed = grades.iter().filter(|g| g.status == "passed").count();
    let failed = grades.iter().filter(|g| g.status == "failed").count();
    let errored = grades.iter().filter(|g| g.status == "errored").count();
    let skipped = grades.iter().filter(|g| g.status == "skipped").count();
    let report = FixtureGradingReport {
        schema_version: "axe.vrs.review-fixtures.v1",
        fixtures_root: root.display().to_string(),
        backend: args.backend.clone(),
        fixtures: grades,
        passed,
        failed,
        errored,
        skipped,
    };

    let json = match serde_json::to_string_pretty(&report) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("axe vrs review-fixtures: failed to render grading report: {error}");
            return ExitCode::from(2);
        }
    };
    if let Some(path) = &args.report {
        if let Err(error) = fs::write(path, format!("{json}\n")) {
            eprintln!(
                "axe vrs review-fixtures: failed to write grading report {}: {error}",
                path.display()
            );
            return ExitCode::from(2);
        }
    } else if args.json {
        println!("{json}");
    }

    if args.report.is_some() || !args.json {
        print_fixture_grades(&report);
    }

    if report.errored > 0 {
        ExitCode::from(2)
    } else if report.failed > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn print_fixture_grades(report: &FixtureGradingReport) {
    for grade in &report.fixtures {
        if grade.status == "passed" {
            continue;
        }
        println!("{} {}", grade.status, grade.id);
        if let Some(reason) = &grade.reason {
            println!("  {reason}");
        }
        for missing in &grade.missing {
            println!(
                "  missing {} {} {} ({})",
                missing.rule, missing.severity, missing.artifact, missing.owner
            );
        }
    }
    println!(
        "axe vrs review-fixtures: {} passed, {} failed, {} errored, {} skipped",
        report.passed, report.failed, report.errored, report.skipped
    );
}

fn select_fixtures(root: &Path, requested: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut available = Vec::new();
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to read fixtures root {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read fixture entry: {error}"))?;
        let path = entry.path();
        if path.is_dir() && path.join("fixture.json").is_file() {
            available.push(path);
        }
    }
    available.sort();

    if requested.is_empty() {
        if available.is_empty() {
            return Err(format!("no fixtures found under {}", root.display()));
        }
        return Ok(available);
    }

    let mut selected = Vec::new();
    for id in requested {
        let candidate = root.join(id);
        if !available.contains(&candidate) {
            return Err(format!("unknown fixture: {id}"));
        }
        selected.push(candidate);
    }
    Ok(selected)
}

fn grade_fixture(
    args: &ReviewFixturesArgs,
    fixture: &Path,
    id: &str,
    workspaces: &Path,
) -> FixtureGrade {
    let errored = |reason: String| FixtureGrade {
        id: id.to_string(),
        status: "errored",
        assertion_mode: None,
        matched: Vec::new(),
        missing: Vec::new(),
        reason: Some(reason),
    };

    let manifest = match read_json(&fixture.join("fixture.json")) {
        Ok(manifest) => manifest,
        Err(error) => return errored(error),
    };
    let assertion_mode = manifest
        .get("assertion_mode")
        .and_then(Value::as_str)
        .map(str::to_string);

    let assertions_path = fixture.join("assertions.json");
    if !assertions_path.is_file() {
        return FixtureGrade {
            id: id.to_string(),
            status: "skipped",
            assertion_mode,
            matched: Vec::new(),
            missing: Vec::new(),
            reason: Some(
                "fixture declares no assertions.json minimum findings to grade against".to_string(),
            ),
        };
    }
    let assertions = match read_json(&assertions_path).and_then(|value| minimum_findings(&value)) {
        Ok(assertions) => assertions,
        Err(error) => return errored(error),
    };

    let prompt = match fixture_asset(fixture, &manifest, "prompt_ref") {
        Ok(path) => path,
        Err(error) => return errored(error),
    };
    let schema = match fixture_asset(fixture, &manifest, "schema_ref") {
        Ok(path) => path,
        Err(error) => return errored(error),
    };

    // The workspace holds `input/` at its root, so the fixture-relative artifact
    // paths in assertions.json are exactly the paths a review reports relative to
    // the coding agent's cwd.
    let workspace = workspaces.join(id);
    let input = fixture.join("input");
    if !input.is_dir() {
        return errored(format!("fixture has no input/ tree: {}", input.display()));
    }
    if let Err(error) = copy_dir_all(&input, &workspace.join("input")) {
        return errored(format!("failed to materialize fixture input: {error}"));
    }
    let workspace = match fs::canonicalize(&workspace) {
        Ok(workspace) => workspace,
        Err(error) => return errored(format!("failed to resolve eval workspace: {error}")),
    };
    let envelope_path = workspaces.join(format!("{id}.result.json"));

    let invocation = ReviewInvocation {
        coding_agent: &args.coding_agent,
        backend: args.backend.as_deref(),
        timeout_seconds: args.timeout_seconds,
        root: &workspace,
        workspace: &workspace,
        prompt: &prompt,
        schema: &schema,
        profile: args.profile,
        final_output: Some(&envelope_path),
    };
    let mut plan = match prepare_review(&invocation) {
        Ok(plan) => plan,
        Err(error) => return errored(error.message),
    };
    let output = match plan.command.output() {
        Ok(output) => output,
        Err(error) => {
            return errored(format!(
                "failed to start CAIC executable {}: {error}",
                args.coding_agent.display()
            ))
        }
    };
    if !output.status.success() {
        // CAIC reports failures as an error envelope on stdout, so a stderr-only
        // message would hide the actual provider diagnosis.
        return errored(format!(
            "review invocation failed with {}{}{}",
            output
                .status
                .code()
                .map(|code| format!("exit code {code}"))
                .unwrap_or_else(|| "terminated process".to_string()),
            output_tail_suffix(&output.stderr),
            output_tail_suffix(&output.stdout)
        ));
    }

    let envelope = match read_json(&envelope_path) {
        Ok(envelope) => envelope,
        Err(error) => return errored(error),
    };
    let Some(result) = envelope.get("result") else {
        return errored("review envelope has no result field".to_string());
    };
    let findings = match result.get("findings").and_then(Value::as_array) {
        Some(findings) => findings,
        None => return errored("review result has no findings array".to_string()),
    };

    let mut matched = Vec::new();
    let mut missing = Vec::new();
    for assertion in assertions {
        if findings
            .iter()
            .any(|finding| finding_satisfies(finding, &assertion, &workspace))
        {
            matched.push(assertion);
        } else {
            missing.push(assertion);
        }
    }

    FixtureGrade {
        id: id.to_string(),
        status: if missing.is_empty() {
            "passed"
        } else {
            "failed"
        },
        assertion_mode,
        matched,
        missing,
        reason: None,
    }
}

fn finding_satisfies(finding: &Value, assertion: &MinimumFinding, workspace: &Path) -> bool {
    let field = |name: &str| {
        finding
            .get(name)
            .and_then(Value::as_str)
            .unwrap_or_default()
    };
    field("rule") == assertion.rule
        && field("severity") == assertion.severity
        && field("owner") == assertion.owner
        && normalized_artifact(field("artifact"), workspace)
            == normalized_artifact(&assertion.artifact, workspace)
}

/// Make an artifact path comparable without weakening decision 0027's exact
/// `artifact` match: absolute paths inside the eval workspace become
/// workspace-relative, and a leading `./` is dropped. No suffix matching, so a
/// finding routed to the wrong artifact still fails.
fn normalized_artifact(artifact: &str, workspace: &Path) -> String {
    let trimmed = artifact.trim();
    let path = Path::new(trimmed);
    if let Ok(relative) = path.strip_prefix(workspace) {
        return relative.display().to_string();
    }
    trimmed.trim_start_matches("./").to_string()
}

fn minimum_findings(value: &Value) -> Result<Vec<MinimumFinding>, String> {
    let entries = value
        .get("minimum_findings")
        .and_then(Value::as_array)
        .ok_or_else(|| "assertions.json has no minimum_findings array".to_string())?;
    let mut findings = Vec::new();
    for entry in entries {
        let field = |name: &str| {
            entry
                .get(name)
                .and_then(Value::as_str)
                .map(str::to_string)
                .ok_or_else(|| format!("minimum finding is missing `{name}`"))
        };
        findings.push(MinimumFinding {
            rule: field("rule")?,
            severity: field("severity")?,
            artifact: field("artifact")?,
            owner: field("owner")?,
        });
    }
    if findings.is_empty() {
        return Err("assertions.json declares no minimum findings".to_string());
    }
    Ok(findings)
}

fn fixture_asset(fixture: &Path, manifest: &Value, field: &str) -> Result<PathBuf, String> {
    let reference = manifest
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("fixture.json is missing `{field}`"))?;
    fs::canonicalize(fixture.join(reference))
        .map_err(|error| format!("`{field}` does not resolve: {reference}: {error}"))
}

fn read_json(path: &Path) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn copy_dir_all(source: &Path, destination: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let target = destination.join(entry.file_name());
        if path.is_dir() {
            copy_dir_all(&path, &target)?;
        } else {
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

fn preflight_review_backend(
    coding_agent: &Path,
    requested_backend: Option<&str>,
) -> Result<(), String> {
    let output = Command::new(coding_agent)
        .arg("capabilities")
        .arg("--json")
        .output()
        .map_err(|error| {
            format!(
                "failed to start CAIC executable {} for capabilities preflight: {error}",
                coding_agent.display()
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "CAIC capabilities preflight failed with {}{}",
            output
                .status
                .code()
                .map(|code| format!("exit code {code}"))
                .unwrap_or_else(|| "terminated process".to_string()),
            output_tail_suffix(&output.stderr)
        ));
    }

    let capabilities: Value = serde_json::from_slice(&output.stdout).map_err(|error| {
        format!(
            "CAIC capabilities preflight did not return JSON: {error}{}",
            output_tail_suffix(&output.stderr)
        )
    })?;
    validate_review_capabilities(&capabilities, requested_backend)
}

fn validate_review_capabilities(
    capabilities: &Value,
    requested_backend: Option<&str>,
) -> Result<(), String> {
    if capabilities.get("schema_version").and_then(Value::as_str)
        != Some("coding_agent.capabilities.v1")
    {
        return Err(
            "CAIC capabilities preflight returned unsupported schema_version; expected coding_agent.capabilities.v1"
                .to_string(),
        );
    }
    let backend_id = match requested_backend {
        Some(backend) => backend.to_string(),
        None => capabilities
            .get("default_backend")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                "CAIC capabilities preflight omitted default_backend and no --backend was provided"
                    .to_string()
            })?
            .to_string(),
    };
    let backend = capabilities
        .get("backends")
        .and_then(Value::as_array)
        .and_then(|backends| {
            backends
                .iter()
                .find(|backend| backend.get("id").and_then(Value::as_str) == Some(&backend_id))
        })
        .ok_or_else(|| format!("backend {backend_id} is not advertised by CAIC capabilities"))?;

    let mut missing = Vec::new();
    require_capability(backend, "modes", "review", &mut missing);
    require_capability(backend, "permissions", "read-only", &mut missing);
    require_capability(backend, "config_policies", "isolated", &mut missing);
    require_capability(backend, "network_policies", "disabled", &mut missing);
    require_capability(backend, "approval_modes", "never", &mut missing);
    require_capability(backend, "output_formats", "json", &mut missing);
    if backend.get("schema_output").and_then(Value::as_bool) != Some(true) {
        missing.push("schema_output=true".to_string());
    }
    if !capability_array_contains(backend, "schema_enforcement", "adapter-validated")
        && !capability_array_contains(backend, "schema_enforcement", "provider-native")
    {
        missing
            .push("schema_enforcement includes adapter-validated or provider-native".to_string());
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "backend {backend_id} does not satisfy axe vrs review preflight: missing {}",
            missing.join(", ")
        ))
    }
}

fn require_capability(backend: &Value, field: &str, value: &str, missing: &mut Vec<String>) {
    if !capability_array_contains(backend, field, value) {
        missing.push(format!("{field} includes {value}"));
    }
}

fn capability_array_contains(backend: &Value, field: &str, expected: &str) -> bool {
    backend
        .get(field)
        .and_then(Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)))
}

fn output_tail_suffix(output: &[u8]) -> String {
    let text = String::from_utf8_lossy(output);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        String::new()
    } else {
        format!(": {trimmed}")
    }
}

pub fn check_root(
    root: &Path,
    profile: Profile,
) -> Result<CheckReport, Box<dyn std::error::Error>> {
    let root = fs::canonicalize(root)?;
    let markdown_files = markdown_files(&root)?;
    let mut diagnostics = Vec::new();

    for path in &markdown_files {
        check_markdown_links(&root, path, profile, &mut diagnostics)?;
    }

    let decision_dir = meta_vrs_decision_dir(&root);
    if decision_dir.is_dir() {
        check_meta_decision_shape(&root, &decision_dir, profile, &mut diagnostics)?;
    }
    check_companion_directories(&root, profile, &mut diagnostics)?;

    Ok(CheckReport {
        schema_version: "axe.vrs.check.v1",
        root: root.display().to_string(),
        profile: match profile {
            Profile::Local => "local",
            Profile::Strict => "strict",
        }
        .to_string(),
        diagnostics,
    })
}

pub fn graph_root(root: &Path) -> Result<GraphReport, Box<dyn std::error::Error>> {
    let root = fs::canonicalize(root)?;
    let markdown_files = markdown_files(&root)?;
    let mut nodes = BTreeSet::new();
    let mut edges = BTreeSet::new();

    for path in &markdown_files {
        let relative = relative_display(&root, path);
        let file_id = graph_file_id(&relative);
        nodes.insert(GraphNode {
            id: file_id.clone(),
            kind: "file".to_string(),
            title: path
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap_or(relative.as_str())
                .to_string(),
            path: relative.clone(),
            status: "active".to_string(),
            refs: Vec::new(),
            refines: Vec::new(),
            evidence: Vec::new(),
        });

        let content = fs::read_to_string(path)?;
        for id in structured_ids_outside_code(&content) {
            nodes.insert(GraphNode {
                id: id.id.clone(),
                kind: graph_id_kind(&id.id).to_string(),
                title: id.title,
                path: relative.clone(),
                status: "active".to_string(),
                refs: id.refs,
                refines: id.refines,
                evidence: vec![id.evidence],
            });
            edges.insert(GraphEdge {
                source: file_id.clone(),
                target: id.id,
                kind: "contains".to_string(),
                path: relative.clone(),
                evidence: "structured-id".to_string(),
            });
        }

        for link in markdown_links_outside_code(&content) {
            let Some(target) = normalized_local_link_target(&link) else {
                continue;
            };
            let (file_part, _anchor_part) = split_anchor(&target);
            if file_part.is_empty() {
                continue;
            }
            let target_path = path.parent().unwrap_or(&root).join(file_part);
            let Ok(target_path) = fs::canonicalize(target_path) else {
                continue;
            };
            if !target_path.starts_with(&root) || target_path.extension() != Some(OsStr::new("md"))
            {
                continue;
            }
            let target_relative = relative_display(&root, &target_path);
            edges.insert(GraphEdge {
                source: file_id.clone(),
                target: graph_file_id(&target_relative),
                kind: "markdown_link".to_string(),
                path: relative.clone(),
                evidence: target,
            });
        }

        for wikilink in wikilinks_outside_code(&content) {
            let target = format!("wiki:{wikilink}");
            nodes.insert(GraphNode {
                id: target.clone(),
                kind: "wikilink".to_string(),
                title: wikilink.clone(),
                path: String::new(),
                status: "unresolved".to_string(),
                refs: Vec::new(),
                refines: Vec::new(),
                evidence: Vec::new(),
            });
            edges.insert(GraphEdge {
                source: file_id.clone(),
                target,
                kind: "wikilink".to_string(),
                path: relative.clone(),
                evidence: format!("[[{wikilink}]]"),
            });
        }
    }

    Ok(GraphReport {
        schema_version: "axe.vrs.graph.v0",
        root: root.display().to_string(),
        nodes: nodes.into_iter().collect(),
        edges: edges.into_iter().collect(),
    })
}

// Corpus-relative only. The old second branch guessed `context/vrs/.decisions` to
// cover being handed a repository root instead of a corpus root — a guess that was
// silently wrong for any repository laid out differently, and that let a misaimed
// invocation look like a clean one. Pointing this at a corpus is the caller's job.
fn meta_vrs_decision_dir(root: &Path) -> PathBuf {
    root.join(".decisions")
}

fn check_markdown_links(
    root: &Path,
    path: &Path,
    profile: Profile,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let anchors = anchors_for(&content);
    for link in markdown_links_outside_code(&content) {
        let Some(target) = normalized_local_link_target(&link) else {
            continue;
        };
        let (file_part, anchor_part) = split_anchor(&target);
        if file_part.is_empty() {
            if !anchor_part.is_empty() && !anchors.contains(anchor_part) {
                diagnostics.push(diagnostic(
                    root,
                    path,
                    "VRS.ENF.link.local-target",
                    format!("Local anchor `#{anchor_part}` does not resolve."),
                    "Update the anchor or heading in this file.",
                    link_severity(profile),
                ));
            }
            continue;
        }

        let target_path = path.parent().unwrap_or(root).join(file_part);
        if !target_path.exists() {
            diagnostics.push(diagnostic(
                root,
                path,
                "VRS.ENF.link.local-target",
                format!("Markdown link target `{target}` does not exist."),
                "Update the link target or add the referenced artifact.",
                link_severity(profile),
            ));
            continue;
        }

        if !anchor_part.is_empty() && target_path.is_file() {
            let target_content = fs::read_to_string(&target_path).unwrap_or_default();
            let target_anchors = anchors_for(&target_content);
            if !target_anchors.contains(anchor_part) {
                diagnostics.push(diagnostic(
                    root,
                    path,
                    "VRS.ENF.link.local-target",
                    format!("Markdown link anchor `{target}` does not resolve."),
                    "Update the anchor or add the referenced heading.",
                    link_severity(profile),
                ));
            }
        }
    }
    Ok(())
}

fn check_meta_decision_shape(
    root: &Path,
    decision_dir: &Path,
    profile: Profile,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in markdown_files_direct(decision_dir)? {
        let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
        if !valid_decision_filename(file_name) {
            diagnostics.push(decision_shape_diagnostic(
                root,
                &path,
                format!("Decision filename `{file_name}` must match `000N-<slug>.md`."),
                "Rename the decision record with the next durable numeric prefix.",
                profile,
            ));
        }

        let content = fs::read_to_string(&path)?;
        let sections = sections(&content);

        match status_line(&content) {
            Some(status) if valid_status(status) => {}
            Some(status) => diagnostics.push(decision_shape_diagnostic(
                root,
                &path,
                format!(
                    "Decision status `{status}` is not accepted, deprecated, or superseded by <id>."
                ),
                "Use an accepted decision status.",
                profile,
            )),
            None => diagnostics.push(decision_shape_diagnostic(
                root,
                &path,
                "Decision record is missing `Status:`.".to_string(),
                "Add `Status: accepted`, `Status: deprecated`, or `Status: superseded by <id>`.",
                profile,
            )),
        }

        for heading in ["Context", "Evidence and Argument", "Options", "Decision"] {
            match sections.iter().find(|section| section.heading == heading) {
                Some(section) if !section.body.trim().is_empty() => {}
                Some(_) => diagnostics.push(decision_shape_diagnostic(
                    root,
                    &path,
                    format!("Decision section `## {heading}` is empty."),
                    "Add the required decision content or keep the record proposed.",
                    profile,
                )),
                None => diagnostics.push(decision_shape_diagnostic(
                    root,
                    &path,
                    format!("Decision record is missing `## {heading}`."),
                    "Add the required decision section or keep the record proposed.",
                    profile,
                )),
            }
        }

        if let Some(options) = sections.iter().find(|section| section.heading == "Options") {
            let option_rows = option_rows(&options.body);
            if option_rows.len() < 2 {
                diagnostics.push(decision_shape_diagnostic(
                    root,
                    &path,
                    "Options section must include at least two option rows.".to_string(),
                    "Use an `Option | Tradeoffs` table with the real alternatives considered.",
                    profile,
                ));
            }
            for (option, tradeoffs) in option_rows {
                if option.trim().is_empty() || tradeoffs.trim().is_empty() {
                    diagnostics.push(decision_shape_diagnostic(
                        root,
                        &path,
                        "Options table contains an empty option or tradeoff cell.".to_string(),
                        "Fill each option row with the option name and its tradeoffs.",
                        profile,
                    ));
                }
            }
        }
    }
    Ok(())
}

fn check_companion_directories(
    root: &Path,
    profile: Profile,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), Box<dyn std::error::Error>> {
    let companion_dirs = companion_dirs(root)?;
    for dir in companion_dirs {
        if is_semantic_review_fixture_input(root, &dir) {
            continue;
        }
        match dir.file_name().and_then(OsStr::to_str) {
            Some(".proposed")
                if dir.parent().and_then(Path::file_name) == Some(OsStr::new(".decisions")) =>
            {
                check_proposed_decisions(root, &dir, diagnostics)?;
            }
            Some(".delta") => check_delta_shape(root, &dir, diagnostics)?,
            Some(".experiments") => check_experiment_shape(root, &dir, profile, diagnostics)?,
            Some(".reference") => check_reference_shape(root, &dir, profile, diagnostics)?,
            _ => {}
        }
    }
    Ok(())
}

fn check_proposed_decisions(
    root: &Path,
    proposed_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in markdown_files_direct(proposed_dir)? {
        diagnostics.push(diagnostic(
            root,
            &path,
            "VRS.ENF.proposed-decision",
            "Proposed decision records are PR-local and must not merge.".to_string(),
            "Accept the decision, fold it into requirements/spec, move it to open questions, or delete it before merge.",
            Severity::Error,
        ));
    }
    Ok(())
}

fn check_delta_shape(
    root: &Path,
    delta_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in markdown_files_direct(delta_dir)? {
        let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
        if !valid_delta_filename(file_name) {
            diagnostics.push(diagnostic(
                root,
                &path,
                "VRS.ENF.delta-shape",
                format!("Delta filename `{file_name}` must match `DELTA-001-<slug>.md`."),
                "Rename the delta with a stable `DELTA-NNN-<slug>.md` identifier.",
                Severity::Error,
            ));
        }

        let content = fs::read_to_string(&path)?;
        match status_line(&content) {
            Some("open") => {}
            Some(status) => diagnostics.push(diagnostic(
                root,
                &path,
                "VRS.ENF.delta-shape",
                format!("Delta status `{status}` is not `open`."),
                "Keep only open delta records; close resolved deltas by deleting the file.",
                Severity::Error,
            )),
            None => diagnostics.push(diagnostic(
                root,
                &path,
                "VRS.ENF.delta-shape",
                "Delta record is missing `Status: open`.".to_string(),
                "Add `Status: open` or delete the delta if it is resolved.",
                Severity::Error,
            )),
        }

        let sections = sections(&content);
        for heading in [
            "Divergence",
            "VRS",
            "Implementation",
            "Direction",
            "Resolution Signal",
        ] {
            require_section(
                root,
                &path,
                &sections,
                heading,
                "VRS.ENF.delta-shape",
                "Fill the required delta section or delete the stale delta.",
                Severity::Error,
                diagnostics,
            );
        }

        if let Some(direction) = sections
            .iter()
            .find(|section| section.heading == "Direction")
        {
            let value = direction.body.trim();
            if !matches!(value, "update implementation" | "update VRS" | "decide") {
                diagnostics.push(diagnostic(
                    root,
                    &path,
                    "VRS.ENF.delta-shape",
                    format!("Delta direction `{value}` must be `update implementation`, `update VRS`, or `decide`."),
                    "Set `## Direction` to one of the accepted delta direction values.",
                    Severity::Error,
                ));
            }
        }
    }
    Ok(())
}

fn check_experiment_shape(
    root: &Path,
    experiment_dir: &Path,
    profile: Profile,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in markdown_files_direct(experiment_dir)? {
        let content = fs::read_to_string(&path)?;
        let sections = sections(&content);
        for heading in ["Question", "Method", "Result", "Conclusion", "VRS Impact"] {
            require_section(
                root,
                &path,
                &sections,
                heading,
                "VRS.ENF.experiment-shape",
                "Fill the required experiment evidence section or move speculative work out of `.experiments/`.",
                companion_shape_severity(profile),
                diagnostics,
            );
        }
    }
    Ok(())
}

fn check_reference_shape(
    root: &Path,
    reference_dir: &Path,
    profile: Profile,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<(), Box<dyn std::error::Error>> {
    for path in markdown_files_direct(reference_dir)? {
        let content = fs::read_to_string(&path)?;
        if source_line(&content).is_none() {
            diagnostics.push(diagnostic(
                root,
                &path,
                "VRS.ENF.reference-shape",
                "Reference record is missing `Source:`.".to_string(),
                "Add the URL, file, command, or system that supplied the reference facts.",
                companion_shape_severity(profile),
            ));
        }

        let sections = sections(&content);
        for heading in ["Relevant Facts", "VRS Impact"] {
            require_section(
                root,
                &path,
                &sections,
                heading,
                "VRS.ENF.reference-shape",
                "Fill the required reference section or delete source material that has no VRS impact.",
                companion_shape_severity(profile),
                diagnostics,
            );
        }
    }
    Ok(())
}

// Eight arguments, one over clippy's threshold. Left as-is deliberately: this crate
// is a lift of `axe vrs`, whose acceptance bar is that it behaves identically, and
// grouping these into a struct is a refactor whose only motivation is a style lint.
// Worth doing later, on its own, where a regression would be attributable.
#[allow(clippy::too_many_arguments)]
fn require_section(
    root: &Path,
    path: &Path,
    sections: &[Section],
    heading: &str,
    rule: &'static str,
    suggested_fix: &str,
    severity: Severity,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match sections.iter().find(|section| section.heading == heading) {
        Some(section) if !section.body.trim().is_empty() => {}
        Some(section) => diagnostics.push(diagnostic(
            root,
            path,
            rule,
            format!("Section `## {}` is empty.", section.heading),
            suggested_fix,
            severity,
        )),
        None => diagnostics.push(diagnostic(
            root,
            path,
            rule,
            format!("Record is missing `## {heading}`."),
            suggested_fix,
            severity,
        )),
    }
}

fn markdown_files(root: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut out = Vec::new();
    visit_markdown(root, root, &mut out)?;
    out.sort();
    Ok(out)
}

fn companion_dirs(root: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut out = Vec::new();
    visit_companion_dirs(root, &mut out)?;
    out.sort();
    Ok(out)
}

fn is_semantic_review_fixture_input(root: &Path, path: &Path) -> bool {
    let Ok(relative) = path.strip_prefix(root) else {
        return false;
    };
    let components: Vec<_> = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect();
    components.len() >= 4
        && components[0] == "15-evaluation"
        && components[1] == "semantic-review"
        && components
            .iter()
            .any(|component| component.as_ref() == "input")
}

fn markdown_files_direct(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension() == Some(OsStr::new("md")) {
            out.push(path);
        }
    }
    out.sort();
    Ok(out)
}

/// The directory the review agent is given to work in, and the boundary its target
/// artifacts may not escape.
///
/// This is still repository-scoped rather than corpus-scoped: a reviewer reasoning
/// about a corpus needs the repository around it. The `context/vrs` sentinel that
/// used to back this up is gone — it named one repository's layout, and it was only
/// ever reached when `.git` was absent. Falling back to the corpus root keeps that
/// no-`.git` case working without the tool having to know any repository's shape.
fn review_workspace(root: &Path) -> PathBuf {
    for ancestor in root.ancestors() {
        if ancestor.join(".git").exists() {
            return ancestor.to_path_buf();
        }
    }
    root.to_path_buf()
}

/// Resolve an enforcement asset that belongs to the corpus.
///
/// Corpus-relative, not repository-relative: `review` reads these from the
/// filesystem at run time, so they must travel with the corpus they describe.
/// Nothing is compiled in, despite what the old error text claimed — there is no
/// `include_str!` here and never was.
fn corpus_asset(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let candidate = root.join(relative);
    if candidate.is_file() {
        return Ok(candidate);
    }
    Err(format!(
        "missing review asset {relative} under corpus {}",
        root.display()
    ))
}

fn automated_context_indicator() -> Option<&'static str> {
    [
        "CI",
        "GITHUB_ACTIONS",
        "BUILDKITE",
        "GITLAB_CI",
        "CIRCLECI",
        "JENKINS_URL",
        "TEAMCITY_VERSION",
        "TF_BUILD",
        "CONTINUOUS_INTEGRATION",
        "CODEBUILD_BUILD_ID",
        "DRONE",
        "PRE_COMMIT",
    ]
    .into_iter()
    .find(|&name| std::env::var(name).is_ok_and(|value| !value.is_empty() && value != "false"))
}

fn visit_markdown(
    root: &Path,
    dir: &Path,
    out: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
        if name == ".git" || name == "target" || name == "node_modules" {
            continue;
        }
        if path.is_dir() {
            // Semantic-review fixture `input/` trees are deliberately broken synthetic
            // artifacts. They are neither real VRS artifacts for deterministic checks nor
            // normative review context; collecting them would ship planted smells to the
            // provider as genuine VRS.
            if is_semantic_review_fixture_input(root, &path) {
                continue;
            }
            visit_markdown(root, &path, out)?;
        } else if path.extension() == Some(OsStr::new("md")) {
            out.push(path);
        }
    }
    Ok(())
}

fn visit_companion_dirs(
    dir: &Path,
    out: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
        if name == ".git" || name == "target" || name == "node_modules" {
            continue;
        }
        if matches!(name, ".proposed" | ".delta" | ".experiments" | ".reference") {
            out.push(path.clone());
        }
        visit_companion_dirs(&path, out)?;
    }
    Ok(())
}

fn markdown_links_outside_code(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut in_fence = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        links.extend(markdown_links_in_line(line));
    }
    links
}

fn wikilinks_outside_code(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut in_fence = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        links.extend(wikilinks_in_line(line));
    }
    links.sort();
    links.dedup();
    links
}

fn wikilinks_in_line(line: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut index = 0;
    while let Some(start) = line[index..].find("[[").map(|offset| index + offset) {
        let link_start = start + 2;
        let Some(end) = line[link_start..]
            .find("]]")
            .map(|offset| link_start + offset)
        else {
            break;
        };
        let raw = line[link_start..end].trim();
        let without_alias = raw
            .split_once('|')
            .map(|(target, _)| target)
            .unwrap_or(raw)
            .trim();
        let target = without_alias
            .split_once('#')
            .map(|(target, _)| target)
            .unwrap_or(without_alias)
            .trim();
        if !target.is_empty() {
            links.push(target.to_string());
        }
        index = end + 2;
    }
    links
}

#[derive(Debug)]
struct StructuredId {
    id: String,
    title: String,
    refs: Vec<String>,
    refines: Vec<String>,
    evidence: String,
}

fn structured_ids_outside_code(content: &str) -> Vec<StructuredId> {
    let mut ids = Vec::new();
    let mut in_fence = false;
    for line in content.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        if let Some(id) = structured_id_in_line(line) {
            ids.push(id);
        }
    }
    ids
}

fn structured_id_in_line(line: &str) -> Option<StructuredId> {
    let start = line.find("**")? + 2;
    let end = line[start..].find("**").map(|offset| start + offset)?;
    let label = line[start..end].trim().trim_end_matches(':').trim();
    let mut parts = label.splitn(2, char::is_whitespace);
    let id = parts.next()?.trim();
    if !looks_like_vrs_id(id) {
        return None;
    }
    let title = parts.next().unwrap_or("").trim().to_string();
    let rest = &line[end + 2..];
    Some(StructuredId {
        id: id.to_string(),
        title: if title.is_empty() {
            id.to_string()
        } else {
            title
        },
        refs: refs_in_text(rest),
        refines: refines_in_text(rest),
        evidence: line.trim().to_string(),
    })
}

fn looks_like_vrs_id(value: &str) -> bool {
    value.len() >= 2
        && value.chars().any(|ch| ch.is_ascii_digit())
        && value
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '.' || ch == '-')
}

fn refs_in_text(text: &str) -> Vec<String> {
    text.split(|ch: char| {
        ch.is_whitespace() || matches!(ch, ',' | ';' | ':' | '(' | ')' | '[' | ']')
    })
    .filter_map(|part| {
        let candidate = part.trim_matches('.');
        looks_like_vrs_id(candidate).then(|| candidate.to_string())
    })
    .collect()
}

fn refines_in_text(text: &str) -> Vec<String> {
    let Some((_, rest)) = text.split_once("refines:") else {
        return Vec::new();
    };
    refs_in_text(rest)
}

fn graph_id_kind(id: &str) -> &'static str {
    if id.contains("-R") {
        "requirement"
    } else if id.contains("-A") {
        "assumption"
    } else if id.contains("-T") {
        "tradeoff"
    } else if id.starts_with("DQ") || id.contains("-DQ") {
        "design_question"
    } else {
        "id"
    }
}

fn graph_file_id(relative: &str) -> String {
    format!("file:{}", relative.replace('\\', "/"))
}

fn markdown_links_in_line(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut links = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        let Some(open_bracket) = line[index..].find('[').map(|offset| index + offset) else {
            break;
        };
        if open_bracket > 0 && bytes[open_bracket - 1] == b'!' {
            index = open_bracket + 1;
            continue;
        }
        let Some(close_bracket) = line[open_bracket..]
            .find(']')
            .map(|offset| open_bracket + offset)
        else {
            break;
        };
        let paren_start = close_bracket + 1;
        if bytes.get(paren_start) != Some(&b'(') {
            index = close_bracket + 1;
            continue;
        }
        let link_start = paren_start + 1;
        let Some(paren_end) = line[link_start..]
            .find(')')
            .map(|offset| link_start + offset)
        else {
            break;
        };
        links.push(line[link_start..paren_end].trim().to_string());
        index = paren_end + 1;
    }
    links
}

fn normalized_local_link_target(link: &str) -> Option<String> {
    let target = link.split_whitespace().next().unwrap_or("").trim();
    if target.is_empty()
        || target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
        || target.starts_with("tel:")
    {
        return None;
    }
    Some(percent_decode_minimal(target))
}

fn split_anchor(target: &str) -> (&str, &str) {
    match target.split_once('#') {
        Some((file, anchor)) => (file, anchor),
        None => (target, ""),
    }
}

fn percent_decode_minimal(value: &str) -> String {
    value.replace("%20", " ")
}

fn anchors_for(content: &str) -> HashSet<String> {
    let mut anchors = HashSet::new();
    let mut seen = HashSet::new();
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') {
            continue;
        }
        let heading = trimmed.trim_start_matches('#').trim();
        if heading.is_empty() {
            continue;
        }
        let mut anchor = github_anchor(heading);
        let base = anchor.clone();
        let mut index = 1;
        while seen.contains(&anchor) {
            anchor = format!("{base}-{index}");
            index += 1;
        }
        seen.insert(anchor.clone());
        anchors.insert(anchor);
    }
    anchors
}

fn github_anchor(heading: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in heading.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if (ch.is_whitespace() || ch == '-') && !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

#[derive(Debug)]
struct Section {
    heading: String,
    body: String,
}

fn sections(content: &str) -> Vec<Section> {
    let mut sections = Vec::new();
    let mut current: Option<Section> = None;
    for line in content.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(section) = current.take() {
                sections.push(section);
            }
            current = Some(Section {
                heading: heading.trim().to_string(),
                body: String::new(),
            });
        } else if let Some(section) = current.as_mut() {
            section.body.push_str(line);
            section.body.push('\n');
        }
    }
    if let Some(section) = current {
        sections.push(section);
    }
    sections
}

fn status_line(content: &str) -> Option<&str> {
    content.lines().find_map(|line| {
        line.strip_prefix("Status:")
            .map(str::trim)
            .filter(|status| !status.is_empty())
    })
}

fn valid_status(status: &str) -> bool {
    status == "accepted" || status == "deprecated" || status.starts_with("superseded by ")
}

fn valid_decision_filename(file_name: &str) -> bool {
    let Some((prefix, rest)) = file_name.split_once('-') else {
        return false;
    };
    prefix.len() == 4
        && prefix.chars().all(|ch| ch.is_ascii_digit())
        && rest.ends_with(".md")
        && rest
            .trim_end_matches(".md")
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
}

fn valid_delta_filename(file_name: &str) -> bool {
    let Some(rest) = file_name.strip_prefix("DELTA-") else {
        return false;
    };
    let Some((number, slug)) = rest.split_once('-') else {
        return false;
    };
    number.len() == 3
        && number.chars().all(|ch| ch.is_ascii_digit())
        && slug.ends_with(".md")
        && slug
            .trim_end_matches(".md")
            .chars()
            .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
}

fn source_line(content: &str) -> Option<&str> {
    content.lines().find_map(|line| {
        line.strip_prefix("Source:")
            .map(str::trim)
            .filter(|source| !source.is_empty())
    })
}

fn option_rows(body: &str) -> Vec<(String, String)> {
    body.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
                return None;
            }
            let cells: Vec<_> = trimmed
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect();
            if cells.len() < 2 {
                return None;
            }
            let first = cells[0];
            let second = cells[1];
            if first.eq_ignore_ascii_case("option")
                || first
                    .chars()
                    .all(|ch| ch == '-' || ch == ':' || ch.is_whitespace())
            {
                return None;
            }
            Some((first.to_string(), second.to_string()))
        })
        .collect()
}

fn decision_shape_diagnostic(
    root: &Path,
    path: &Path,
    evidence: String,
    suggested_fix: &str,
    _profile: Profile,
) -> Diagnostic {
    diagnostic(
        root,
        path,
        "VRS.ENF.meta-decision-shape",
        evidence,
        suggested_fix,
        Severity::Error,
    )
}

fn link_severity(profile: Profile) -> Severity {
    match profile {
        Profile::Local => Severity::Warning,
        Profile::Strict => Severity::Error,
    }
}

fn companion_shape_severity(profile: Profile) -> Severity {
    match profile {
        Profile::Local => Severity::Warning,
        Profile::Strict => Severity::Error,
    }
}

fn diagnostic(
    root: &Path,
    path: &Path,
    rule: &'static str,
    evidence: String,
    suggested_fix: &str,
    severity: Severity,
) -> Diagnostic {
    Diagnostic {
        schema_version: "axe.vrs.diagnostic.v1",
        kind: "deterministic",
        gate: match severity {
            Severity::Error => "blocking",
            Severity::Warning => "transitional",
            Severity::Info => "advisory",
        },
        severity,
        artifact: relative_display(root, path),
        owner: owner_for(root, path),
        rule,
        evidence,
        suggested_fix: suggested_fix.to_string(),
    }
}

fn relative_display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn owner_for(root: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    relative
        .components()
        .next()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".to_string())
}

fn severity_label(severity: &Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_fixture_passes_strict_checks() {
        let tempdir = tempfile::tempdir().unwrap();
        let root = tempdir.path().join("context/vrs");
        fs::create_dir_all(&root).unwrap();
        write_valid_vrs(&root);

        let report = check_root(&root, Profile::Strict).unwrap();
        assert!(
            report.diagnostics.is_empty(),
            "unexpected diagnostics: {:?}",
            report.diagnostics
        );
    }

    #[test]
    fn broken_links_and_decisions_are_reported() {
        let tempdir = tempfile::tempdir().unwrap();
        let root = tempdir.path().join("context/vrs");
        fs::create_dir_all(&root).unwrap();
        write_valid_vrs(&root);
        fs::write(
            root.join("spec.md"),
            "# Spec\n\nSee [missing](./missing.md) and [bad anchor](./requirements.md#missing).\n",
        )
        .unwrap();
        fs::write(
            root.join(".decisions/0001-bad.md"),
            "# Bad\n\nStatus: maybe\n\n## Context\n\nx\n\n## Options\n\n| Option | Tradeoffs |\n| --- | --- |\n| A | x |\n",
        )
        .unwrap();

        let report = check_root(&root, Profile::Strict).unwrap();
        let rules: Vec<_> = report.diagnostics.iter().map(|d| d.rule).collect();
        assert!(rules.contains(&"VRS.ENF.link.local-target"));
        assert!(rules.contains(&"VRS.ENF.meta-decision-shape"));
        assert!(report
            .diagnostics
            .iter()
            .all(|d| d.severity == Severity::Error));
    }

    #[test]
    fn local_profile_still_blocks_for_decision_shape() {
        let tempdir = tempfile::tempdir().unwrap();
        let root = tempdir.path().join("context/vrs");
        fs::create_dir_all(root.join(".decisions")).unwrap();
        fs::write(
            root.join(".decisions/0001-bad.md"),
            "# Bad\n\nStatus: maybe\n",
        )
        .unwrap();

        let report = check_root(&root, Profile::Local).unwrap();
        assert!(report
            .diagnostics
            .iter()
            .any(|d| { d.rule == "VRS.ENF.meta-decision-shape" && d.severity == Severity::Error }));
    }

    // The extraction's acceptance bar is that `axe vrs` behaves identically, and the
    // only thing holding that up is the caller keeping its own default. Locked here
    // because a regression is silent: the wrong root still exits 0.
    #[test]
    fn an_absent_argument_falls_back_to_the_callers_layout() {
        let axe = Defaults::corpus_root("context/vrs");
        assert_eq!(axe.root_or_default(None), PathBuf::from("context/vrs"));
        assert_eq!(
            axe.fixtures_or_default(None),
            PathBuf::from("context/vrs/15-evaluation/semantic-review")
        );

        // Standalone `intent` checks the corpus it is run in.
        let standalone = Defaults::default();
        assert_eq!(standalone.root_or_default(None), PathBuf::from("."));
        assert_eq!(
            standalone.fixtures_or_default(None),
            PathBuf::from("./15-evaluation/semantic-review")
        );
    }

    #[test]
    fn an_explicit_argument_always_beats_the_default() {
        let defaults = Defaults::corpus_root("context/vrs");
        let explicit = PathBuf::from("/somewhere/else");
        assert_eq!(defaults.root_or_default(Some(explicit.clone())), explicit);
        assert_eq!(
            defaults.fixtures_or_default(Some(explicit.clone())),
            explicit
        );
    }

    // Covers the branch that replaced the `context/vrs` sentinel. It is only ever
    // reached where there is no `.git` — a Nix build sandbox or a vendored source
    // tree — so it is invisible to any interactive run.
    #[test]
    fn review_workspace_falls_back_to_the_corpus_when_there_is_no_git() {
        let tempdir = tempfile::tempdir().unwrap();
        let corpus = tempdir.path().join("some/corpus");
        fs::create_dir_all(&corpus).unwrap();

        assert_eq!(review_workspace(&corpus), corpus);
    }

    #[test]
    fn review_workspace_prefers_the_enclosing_repository() {
        let tempdir = tempfile::tempdir().unwrap();
        let repo = fs::canonicalize(tempdir.path()).unwrap();
        let corpus = repo.join("some/corpus");
        fs::create_dir_all(&corpus).unwrap();
        fs::create_dir(repo.join(".git")).unwrap();

        assert_eq!(review_workspace(&corpus), repo);
    }

    // The assets travel with the corpus rather than the repository: that co-location
    // is the reason the tool was moved next to the corpus in the first place.
    #[test]
    fn enforcement_assets_resolve_under_the_corpus_not_the_repository() {
        let tempdir = tempfile::tempdir().unwrap();
        let repo = tempdir.path();
        let corpus = repo.join("intent");
        fs::create_dir_all(corpus.join("16-enforcement")).unwrap();
        fs::write(corpus.join(REVIEW_PROMPT_ASSET), "# prompt\n").unwrap();

        assert_eq!(
            corpus_asset(&corpus, REVIEW_PROMPT_ASSET).unwrap(),
            corpus.join(REVIEW_PROMPT_ASSET)
        );

        // A repository-relative copy must NOT satisfy a corpus-relative lookup.
        fs::create_dir_all(repo.join("context/vrs/16-enforcement")).unwrap();
        fs::write(repo.join("context/vrs").join(REVIEW_SCHEMA_ASSET), "{}").unwrap();
        assert!(corpus_asset(&corpus, REVIEW_SCHEMA_ASSET).is_err());
    }

    fn write_valid_vrs(root: &Path) {
        fs::create_dir(root.join(".decisions")).unwrap();
        fs::write(
            root.join("requirements.md"),
            "# Requirements\n\n## Context\n",
        )
        .unwrap();
        fs::write(
            root.join("spec.md"),
            "# Spec\n\nSee [requirements](./requirements.md#context).\n",
        )
        .unwrap();
        fs::write(
            root.join(".decisions/0001-valid.md"),
            "# Valid\n\nStatus: accepted\n\n## Context\n\nA choice was required.\n\n## Evidence and Argument\n\nA fixture proves the mechanical shape.\n\n## Options\n\n| Option | Tradeoffs |\n| --- | --- |\n| A | Simple but narrow. |\n| B | Broader but expensive. |\n\n## Decision\n\nChoose A because it is enough for this fixture.\n",
        )
        .unwrap();
    }
}
