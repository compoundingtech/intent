//! Integration tests ported from `flakes/axe/tests/intent_check.rs` in
//! `schickling/dotfiles`, where the suite stayed behind when the checker was lifted
//! into this crate. They drive the built `intent` binary end to end and are the
//! differential oracle for the extraction.
//!
//! **10 of the original 19 travelled. The other 9 did not, and this is the record of
//! which and why** — a suite that looks complete because the hard cases were quietly
//! dropped is worse than a smaller honest one.
//!
//! The 9 omitted tests all exercise `review` or `review-fixtures` through the CAIC
//! runner, which is a SEPARATE binary (`coding-agent`, a shim over `axe::caic`) owned
//! by `axe`. This crate does not build one, and lifting `caic` here would invert the
//! dependency — `axe` consumes `intent`, not the other way round. Their assertions are
//! on the CAIC envelope itself (`coding_agent.result.v1`, `run.context_files`,
//! `run.permission.effective`), so a stub could only make them pass by reimplementing
//! the collaborator under test:
//!
//! - `semantic_review_fixture_inputs_never_enter_the_review_packet`
//! - `review_invokes_caic_read_only_with_generated_diagnostics`
//! - `review_supports_claude_backend_without_retired_tool_names`
//! - `review_report_writes_result_file_without_stdout`
//! - `review_fixtures_grades_a_result_that_meets_the_minimum_assertions`
//! - `review_fixtures_fails_a_misrouted_or_downgraded_finding`
//! - `review_fixtures_matches_workspace_absolute_artifact_paths`
//! - `review_fixtures_skips_fixtures_without_minimum_assertions`
//! - `review_fixtures_refuses_likely_automated_context`
//!
//! Those 9 remain green in `axe`'s own suite, so the behaviour is still covered — but
//! only for as long as `axe` keeps consuming this crate. Closing that gap needs a CAIC
//! boundary this crate can drive on its own; it is not closed by adding a fake here.
//!
//! The two `review_*` tests that DID travel are the ones that refuse before CAIC is
//! ever executed, so no runner is needed to reach their assertions.

use serde_json::Value;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Output};

struct Harness {
    _tempdir: tempfile::TempDir,
    intent: PathBuf,
    /// Path handed to `--coding-agent`. The CAIC runner is a separate binary owned by
    /// `axe`, and this crate does not build one, so this deliberately points at a file
    /// that does not exist: the only test using it asserts that `review` refuses BEFORE
    /// it would ever be executed. If a test ever needs this path to run, that test does
    /// not belong here — see the omissions noted at the bottom of this file.
    coding_agent: PathBuf,
    repo: PathBuf,
}

impl Harness {
    fn new() -> Self {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let repo = tempdir.path().join("repo");
        fs::create_dir_all(repo.join("context/intent/.decisions")).expect("repo");
        fs::create_dir_all(repo.join("context/intent/16-enforcement")).expect("enforcement");
        fs::write(repo.join("context/intent/spec.md"), "# Spec\n").expect("spec");
        fs::write(
            repo.join("context/intent/16-enforcement/review-prompt.md"),
            "Return a schema-valid fake Intent review result.",
        )
        .expect("review prompt");
        fs::write(
            repo.join("context/intent/16-enforcement/review-result.schema.json"),
            r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "required": ["schema_version", "summary", "findings"],
  "properties": {
    "schema_version": { "const": "axe.intent.review.v1" },
    "summary": { "type": "string" },
    "findings": { "type": "array" }
  },
  "additionalProperties": true
}
"#,
        )
        .expect("review schema");
        fs::write(
            repo.join("context/intent/.decisions/0001-good.md"),
            r#"# Good Decision

Status: accepted

## Context

The context is concrete.

## Evidence and Argument

The evidence is named.

## Options

| Option | Tradeoffs |
| --- | --- |
| A | Simpler, but narrower. |
| B | Broader, but costlier. |

## Decision

Choose A because it fits the current scope.
"#,
        )
        .expect("decision");

        Self {
            _tempdir: tempdir,
            intent: PathBuf::from(env!("CARGO_BIN_EXE_intent")),
            coding_agent: repo.join("no-caic-runner-is-built-by-this-crate"),
            repo,
        }
    }

    fn check(&self, args: &[&str]) -> Output {
        Command::new(&self.intent)
            .arg("check")
            .arg(self.repo.join("context/intent"))
            .args(args)
            .output()
            .expect("intent check")
    }

    fn check_at(&self, path: &Path, args: &[&str]) -> Output {
        Command::new(&self.intent)
            .arg("check")
            .arg(path)
            .args(args)
            .output()
            .expect("intent check")
    }

    fn graph(&self, args: &[&str]) -> Output {
        Command::new(&self.intent)
            .arg("graph")
            .arg(self.repo.join("context/intent"))
            .args(args)
            .output()
            .expect("intent graph")
    }
}

fn stdout_json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout json")
}

#[test]
fn help_advertises_the_default_root_the_run_will_use() {
    let h = Harness::new();
    for (subcommand, expected) in [
        ("check", "[default: .]"),
        ("graph", "[default: .]"),
        ("review", "[default: .]"),
        (
            "review-fixtures",
            "[default: ./15-evaluation/semantic-review]",
        ),
    ] {
        let output = Command::new(&h.intent)
            .arg(subcommand)
            .arg("--help")
            .output()
            .expect("intent --help");
        assert!(output.status.success());
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(
            help.contains(expected),
            "`{subcommand} --help` must advertise {expected}; help:\n{help}"
        );
    }
}

fn write_executable(path: &Path, body: &str) -> PathBuf {
    let body = body.replacen(
        "#!/usr/bin/env bash",
        &format!("#!{}", bash_path().display()),
        1,
    );
    let candidate = path.with_extension("candidate");
    fs::write(&candidate, body).expect("write fake provider candidate");
    let mut permissions = fs::metadata(&candidate)
        .expect("candidate metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&candidate, permissions).expect("chmod fake provider candidate");
    fs::rename(&candidate, path).expect("atomically publish fake provider");
    path.to_path_buf()
}

fn bash_path() -> PathBuf {
    std::env::var_os("PATH")
        .and_then(|paths| {
            std::env::split_paths(&paths)
                .map(|path| path.join("bash"))
                .find(|path| path.exists())
        })
        .expect("bash on PATH")
}

#[test]
fn graph_json_exposes_files_ids_links_and_wikilinks() {
    let h = Harness::new();
    fs::write(
        h.repo.join("context/intent/requirements.md"),
        "# Requirements\n\n- **AXE.INTENT-R08 Graph command:** emit graph JSON; refines: INTENT-R27.\n",
    )
    .expect("requirements");
    fs::write(
        h.repo.join("context/intent/spec.md"),
        "# Spec\n\nSee [requirements](./requirements.md) and [[Graph Backlog|graph work]].\n\n```text\n[[IgnoredInFence]]\n```\n",
    )
    .expect("spec");

    let output = h.graph(&["--json"]);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let graph = stdout_json(&output);
    assert_eq!(graph["schema_version"], "axe.intent.graph.v0");

    let nodes = graph["nodes"].as_array().unwrap();
    assert!(nodes
        .iter()
        .any(|node| { node["id"] == "file:requirements.md" && node["kind"] == "file" }));
    assert!(nodes.iter().any(|node| {
        node["id"] == "AXE.INTENT-R08"
            && node["kind"] == "requirement"
            && node["title"] == "Graph command"
            && node["refines"]
                .as_array()
                .unwrap()
                .iter()
                .any(|id| id == "INTENT-R27")
    }));
    assert!(nodes
        .iter()
        .any(|node| { node["id"] == "wiki:Graph Backlog" && node["kind"] == "wikilink" }));
    assert!(
        !nodes.iter().any(|node| node["id"] == "wiki:IgnoredInFence"),
        "wikilinks in fenced code must not enter the derived graph"
    );

    let edges = graph["edges"].as_array().unwrap();
    assert!(edges.iter().any(|edge| {
        edge["source"] == "file:requirements.md"
            && edge["target"] == "AXE.INTENT-R08"
            && edge["kind"] == "contains"
    }));
    assert!(edges.iter().any(|edge| {
        edge["source"] == "file:spec.md"
            && edge["target"] == "file:requirements.md"
            && edge["kind"] == "markdown_link"
    }));
    assert!(edges.iter().any(|edge| {
        edge["source"] == "file:spec.md"
            && edge["target"] == "wiki:Graph Backlog"
            && edge["kind"] == "wikilink"
    }));
}

#[test]
fn graph_json_records_local_and_external_markdown_links() {
    let h = Harness::new();
    fs::write(
        h.repo.join("context/intent/requirements.md"),
        "# Requirements\n\n## Local target\n",
    )
    .expect("requirements");
    fs::write(
        h.repo.join("context/intent/spec.md"),
        "# Spec\n\nSee π [the requirement](./requirements.md#local-target), [missing](./missing.md), and [Decision 0042](https://github.com/compoundingtech/intent/commit/9561206).\n",
    )
    .expect("spec");

    let output = h.graph(&["--json"]);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let graph = stdout_json(&output);
    let links = graph["references"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|reference| reference["syntax"] == "markdown_link")
        .collect::<Vec<_>>();
    assert_eq!(links.len(), 3);

    let resolved = links
        .iter()
        .find(|reference| reference["written_target"] == "./requirements.md#local-target")
        .unwrap();
    assert_eq!(resolved["label"], "the requirement");
    assert_eq!(resolved["resolution"], "resolved");
    assert_eq!(
        resolved["resolved_target"],
        "context/intent/requirements.md#local-target"
    );
    assert_eq!(resolved["source"]["path"], "context/intent/spec.md");
    assert_eq!(resolved["source"]["start"]["line"], 3);
    assert_eq!(resolved["source"]["start"]["column"], 7);
    assert_eq!(
        resolved["target_locations"][0]["path"],
        "context/intent/requirements.md"
    );
    assert_eq!(resolved["target_locations"][0]["start"]["line"], 3);

    let dangling = links
        .iter()
        .find(|reference| reference["written_target"] == "./missing.md")
        .unwrap();
    assert_eq!(dangling["resolution"], "dangling");
    assert_eq!(dangling["resolved_target"], "context/intent/missing.md");
    assert!(dangling["target_locations"].as_array().unwrap().is_empty());

    let external = links
        .iter()
        .find(|reference| {
            reference["written_target"]
                == "https://github.com/compoundingtech/intent/commit/9561206"
        })
        .unwrap();
    assert_eq!(external["label"], "Decision 0042");
    assert_eq!(external["resolution"], "external");
    assert!(external["target_locations"].as_array().unwrap().is_empty());
    assert!(external.get("resolved_target").is_none());
}

#[test]
fn graph_json_resolves_traces_plain_ids_and_scoped_companion_ordinals() {
    let h = Harness::new();
    fs::create_dir_all(h.repo.join("context/intent/.delta")).expect("delta directory");
    fs::create_dir_all(h.repo.join("context/intent/child-a")).expect("child a");
    fs::create_dir_all(h.repo.join("context/intent/child-b")).expect("child b");
    fs::write(
        h.repo.join("context/intent/requirements.md"),
        "# Requirements\n\n- **AXE.INTENT-R08 Graph references:** emit references.\n- **R02 Local graph references:** resolve in the linked requirements scope.\n",
    )
    .expect("requirements");
    for child in ["child-a", "child-b"] {
        fs::write(
            h.repo
                .join(format!("context/intent/{child}/requirements.md")),
            "# Requirements\n\n- **DUP-R01 Duplicate namespace:** fixture.\n",
        )
        .expect("duplicate requirement");
    }
    for slug in ["first", "second"] {
        fs::write(
            h.repo
                .join(format!("context/intent/.decisions/0002-{slug}.md")),
            format!("# {slug}\n"),
        )
        .expect("duplicate decision ordinal");
    }
    fs::write(
        h.repo.join("context/intent/.delta/DELTA-004-drift.md"),
        "# DELTA-004: Drift\n",
    )
    .expect("delta");
    fs::write(
        h.repo.join("context/intent/spec.md"),
        "# Spec\n\nThis builds on [requirements](./requirements.md).\n\nreq: AXE.INTENT-R08\nAXE.INTENT-R08 and R02 constrain the graph.\nDUP-R01 and Decision 0002 are ambiguous in their respective scopes.\nDecision 0001 and DELTA-4 record the rationale and current drift.\n",
    )
    .expect("spec");

    let output = h.graph(&["--json"]);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let graph = stdout_json(&output);
    let references = graph["references"].as_array().unwrap();

    let trace = references
        .iter()
        .find(|reference| {
            reference["syntax"] == "req_trace" && reference["written_target"] == "AXE.INTENT-R08"
        })
        .unwrap();
    assert_eq!(trace["resolution"], "resolved");
    assert_eq!(trace["normalized_target"], "AXE.INTENT-R08");

    let namespaced = references
        .iter()
        .find(|reference| {
            reference["syntax"] == "id_citation" && reference["written_target"] == "AXE.INTENT-R08"
        })
        .unwrap();
    assert_eq!(namespaced["resolution"], "resolved");
    assert_eq!(namespaced["scope"], "namespace:AXE.INTENT");

    let local = references
        .iter()
        .find(|reference| {
            reference["syntax"] == "id_citation" && reference["written_target"] == "R02"
        })
        .unwrap();
    assert_eq!(local["resolution"], "resolved");
    assert_eq!(local["scope"], "linked:context/intent/requirements.md");

    let ambiguous_id = references
        .iter()
        .find(|reference| {
            reference["syntax"] == "id_citation" && reference["written_target"] == "DUP-R01"
        })
        .unwrap();
    assert_eq!(ambiguous_id["resolution"], "ambiguous");
    assert_eq!(
        ambiguous_id["target_locations"].as_array().unwrap().len(),
        2
    );

    let ambiguous_ordinal = references
        .iter()
        .find(|reference| {
            reference["syntax"] == "ordinal_citation" && reference["written_target"] == "0002"
        })
        .unwrap();
    assert_eq!(ambiguous_ordinal["resolution"], "ambiguous");
    assert_eq!(
        ambiguous_ordinal["target_locations"]
            .as_array()
            .unwrap()
            .len(),
        2
    );

    for (written, normalized, target_suffix) in [
        ("0001", "0001", ".decisions/0001-good.md"),
        ("DELTA-4", "DELTA-004", ".delta/DELTA-004-drift.md"),
    ] {
        let ordinal = references
            .iter()
            .find(|reference| {
                reference["syntax"] == "ordinal_citation" && reference["written_target"] == written
            })
            .unwrap();
        assert_eq!(ordinal["normalized_target"], normalized);
        assert_eq!(ordinal["resolution"], "resolved");
        assert!(
            ordinal["target_locations"][0]["path"]
                .as_str()
                .unwrap()
                .ends_with(target_suffix),
            "{ordinal:#?}"
        );
    }
}

#[test]
fn graph_json_recognizes_all_supported_requirement_definition_forms() {
    let h = Harness::new();
    fs::write(
        h.repo.join("context/intent/requirements.md"),
        "# Requirements\n\n- **A01 Local assumption:** fixture.\n- **R01 Local requirement:** fixture.\n- **T01 Local tradeoff:** fixture.\n- **APP-R02 A multiline\n  requirement title:** fixture.\n- **R03.** Legacy local requirement.\n",
    )
    .expect("requirements");
    fs::write(
        h.repo.join("context/intent/spec.md"),
        "# Spec\n\nThis builds on [requirements](./requirements.md).\n\nAPP-R02 and R03 constrain the graph.\nreq: APP-R02\nreq: APP-R02<br>\nreq: APP-R02<!-- note -->\nreq: <ID>\nreq: <repo-relative-requirements-path>#<ID>\nreq: requirements.md#<ID>\nreq: APP-<ID>\n",
    )
    .expect("spec");

    let output = h.graph(&["--json"]);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let graph = stdout_json(&output);
    let nodes = graph["nodes"].as_array().unwrap();
    for (id, kind, title) in [
        ("A01", "assumption", "Local assumption"),
        ("R01", "requirement", "Local requirement"),
        ("T01", "tradeoff", "Local tradeoff"),
        ("APP-R02", "requirement", "A multiline requirement title"),
        ("R03", "requirement", "R03"),
    ] {
        assert!(
            nodes
                .iter()
                .any(|node| { node["id"] == id && node["kind"] == kind && node["title"] == title }),
            "missing {kind} node {id}"
        );
    }
    assert!(
        !nodes.iter().any(|node| node["id"] == "R03."),
        "legacy punctuation is not part of the requirement ID"
    );

    let references = graph["references"].as_array().unwrap();
    for target in ["APP-R02", "R03"] {
        assert!(
            references.iter().any(|reference| {
                reference["syntax"] == "id_citation"
                    && reference["written_target"] == target
                    && reference["resolution"] == "resolved"
            }),
            "missing resolved citation for {target}"
        );
    }
    let traces = references
        .iter()
        .filter(|reference| reference["syntax"] == "req_trace")
        .collect::<Vec<_>>();
    assert_eq!(
        traces.len(),
        3,
        "template placeholders must not become requirement traces: {traces:#?}"
    );
    assert!(
        traces
            .iter()
            .all(|trace| trace["written_target"] == "APP-R02"),
        "HTML immediately after a complete target must preserve the trace: {traces:#?}"
    );
}

#[test]
fn valid_minimal_intent_tree_passes_json_check() {
    let h = Harness::new();

    let output = h.check(&["--json"]);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = stdout_json(&output);
    assert_eq!(report["schema_version"], "axe.intent.check.v1");
    assert_eq!(report["diagnostics"].as_array().unwrap().len(), 0);
}

#[test]
fn missing_local_markdown_links_are_transitional_locally_and_blocking_in_strict_profile() {
    let h = Harness::new();
    fs::write(
        h.repo.join("context/intent/spec.md"),
        "# Spec\n\nSee [missing](./missing.md).\n",
    )
    .expect("broken link");

    let local = h.check(&["--json"]);
    assert!(local.status.success());
    let local_report = stdout_json(&local);
    assert_eq!(
        local_report["diagnostics"][0]["rule"],
        "INTENT.ENF.link.local-target"
    );
    assert_eq!(local_report["diagnostics"][0]["severity"], "warning");
    assert_eq!(local_report["diagnostics"][0]["gate"], "transitional");

    let strict = h.check(&["--json", "--profile", "strict"]);
    assert_eq!(strict.status.code(), Some(1));
    let strict_report = stdout_json(&strict);
    assert_eq!(strict_report["diagnostics"][0]["severity"], "error");
    assert_eq!(strict_report["diagnostics"][0]["gate"], "blocking");
}

#[test]
fn root_intent_decision_shape_is_blocking() {
    let h = Harness::new();
    fs::write(
        h.repo.join("context/intent/.decisions/0002-bad.md"),
        r#"# Bad Decision

Status:

## Context

Present.

## Options

No comparison table.

## Decision
"#,
    )
    .expect("bad decision");

    let output = h.check(&["--json"]);
    assert_eq!(output.status.code(), Some(1));
    let report = stdout_json(&output);
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["rule"] == "INTENT.ENF.meta-decision-shape"
            && diagnostic["artifact"]
                .as_str()
                .unwrap()
                .ends_with("0002-bad.md")
    }));
}

#[test]
fn decision_shape_is_enforced_when_check_is_aimed_at_the_repository_root() {
    let h = Harness::new();
    fs::write(
        h.repo.join("context/intent/.decisions/0002-bad.md"),
        r#"# Bad Decision

Status:

## Context

Present.

## Options

No comparison table.

## Decision
"#,
    )
    .expect("bad decision");

    let output = h.check_at(&h.repo, &["--json"]);
    let report = stdout_json(&output);
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert!(
        diagnostics.iter().any(|diagnostic| {
            diagnostic["rule"] == "INTENT.ENF.meta-decision-shape"
                && diagnostic["artifact"]
                    .as_str()
                    .unwrap()
                    .ends_with("0002-bad.md")
        }),
        "a malformed decision must be reported when check is aimed at the repository root; diagnostics:\n{}",
        serde_json::to_string_pretty(&report["diagnostics"]).unwrap()
    );
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn proposed_decision_records_are_blocking() {
    let h = Harness::new();
    fs::create_dir_all(h.repo.join("context/intent/.decisions/.proposed")).expect("proposed dir");
    fs::write(
        h.repo
            .join("context/intent/.decisions/.proposed/revisit-scope.md"),
        "# Proposed\n",
    )
    .expect("proposed decision");

    let output = h.check(&["--json"]);
    assert_eq!(output.status.code(), Some(1));
    let report = stdout_json(&output);
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["rule"] == "INTENT.ENF.proposed-decision"
            && diagnostic["severity"] == "error"
            && diagnostic["artifact"]
                .as_str()
                .unwrap()
                .ends_with(".decisions/.proposed/revisit-scope.md")
    }));
}

#[test]
fn delta_record_shape_is_blocking() {
    let h = Harness::new();
    fs::create_dir_all(h.repo.join("context/intent/.delta")).expect("delta dir");
    fs::write(
        h.repo.join("context/intent/.delta/DELTA-001-good.md"),
        r#"# DELTA-001: Good

Status: open

## Divergence

The implementation and Intent differ.

## Intent

See [spec](../spec.md).

## Implementation

Observed in a local check.

## Direction

update Intent

## Resolution Signal

The spec reflects the implementation.
"#,
    )
    .expect("good delta");
    fs::write(
        h.repo.join("context/intent/.delta/delta-bad.md"),
        r#"# Bad Delta

Status: closed

## Divergence

This is stale.
"#,
    )
    .expect("bad delta");

    let output = h.check(&["--json"]);
    assert_eq!(output.status.code(), Some(1));
    let report = stdout_json(&output);
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic["rule"] == "INTENT.ENF.delta-shape"
            && diagnostic["severity"] == "error"
            && diagnostic["artifact"]
                .as_str()
                .unwrap()
                .ends_with(".delta/delta-bad.md")
    }));
    assert!(!diagnostics.iter().any(|diagnostic| {
        diagnostic["artifact"]
            .as_str()
            .unwrap()
            .ends_with(".delta/DELTA-001-good.md")
    }));
}

#[test]
fn semantic_review_fixture_inputs_are_not_treated_as_real_intent_artifacts() {
    let h = Harness::new();
    let fixture_delta = h.repo.join(
        "context/intent/15-evaluation/semantic-review/stale-delta/input/context/stale-delta/.delta",
    );
    fs::create_dir_all(&fixture_delta).expect("fixture delta dir");
    fs::write(
        fixture_delta.join("DELTA-001-intentionally-malformed.md"),
        "# Fixture Delta\n\nStatus: closed\n\n## Divergence\n\nFixture input.\n",
    )
    .expect("fixture delta");

    let output = h.check(&["--json"]);
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = stdout_json(&output);
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert!(diagnostics.iter().all(|diagnostic| {
        !diagnostic["artifact"]
            .as_str()
            .unwrap()
            .contains("semantic-review/stale-delta/input")
    }));
}

#[test]
fn experiment_and_reference_shape_are_transitional_locally_and_blocking_in_strict_profile() {
    let h = Harness::new();
    fs::create_dir_all(h.repo.join("context/intent/.experiments")).expect("experiments dir");
    fs::create_dir_all(h.repo.join("context/intent/.reference")).expect("reference dir");
    fs::write(
        h.repo.join("context/intent/.experiments/smoke.md"),
        "# Smoke\n\n## Question\n\nWhat happens?\n",
    )
    .expect("experiment");
    fs::write(
        h.repo.join("context/intent/.reference/provider.md"),
        "# Provider\n\n## Relevant Facts\n\nFact.\n",
    )
    .expect("reference");

    let local = h.check(&["--json"]);
    assert!(local.status.success());
    let local_report = stdout_json(&local);
    let local_diagnostics = local_report["diagnostics"].as_array().unwrap();
    assert!(local_diagnostics.iter().any(|diagnostic| {
        diagnostic["rule"] == "INTENT.ENF.experiment-shape"
            && diagnostic["severity"] == "warning"
            && diagnostic["gate"] == "transitional"
    }));
    assert!(local_diagnostics.iter().any(|diagnostic| {
        diagnostic["rule"] == "INTENT.ENF.reference-shape"
            && diagnostic["severity"] == "warning"
            && diagnostic["gate"] == "transitional"
    }));

    let strict = h.check(&["--json", "--profile", "strict"]);
    assert_eq!(strict.status.code(), Some(1));
    let strict_report = stdout_json(&strict);
    let strict_diagnostics = strict_report["diagnostics"].as_array().unwrap();
    assert!(strict_diagnostics.iter().any(|diagnostic| {
        diagnostic["rule"] == "INTENT.ENF.experiment-shape"
            && diagnostic["severity"] == "error"
            && diagnostic["gate"] == "blocking"
    }));
    assert!(strict_diagnostics.iter().any(|diagnostic| {
        diagnostic["rule"] == "INTENT.ENF.reference-shape"
            && diagnostic["severity"] == "error"
            && diagnostic["gate"] == "blocking"
    }));
}

#[test]
fn review_refuses_likely_automated_context() {
    let h = Harness::new();

    let output = Command::new(&h.intent)
        .arg("review")
        .arg(h.repo.join("context/intent"))
        .arg("--coding-agent")
        .arg(&h.coding_agent)
        .env("CI", "true")
        .output()
        .expect("intent review");
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("automated context (CI)"),
        "stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn review_refuses_backend_without_review_contract_before_invoking_caic_run() {
    let h = Harness::new();
    let fake_coding_agent = write_executable(
        &h.repo.join("fake-coding-agent"),
        FAKE_CODING_AGENT_UNSUPPORTED_DEFAULT,
    );
    let marker = h.repo.join("caic-run-invoked");

    let output = Command::new(&h.intent)
        .arg("review")
        .arg(h.repo.join("context/intent"))
        .arg("--coding-agent")
        .arg(fake_coding_agent)
        .env("FAKE_CAIC_RUN_MARKER", &marker)
        .env_remove("CI")
        .env_remove("GITHUB_ACTIONS")
        .env_remove("BUILDKITE")
        .output()
        .expect("intent review");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("backend opencode does not satisfy intent review preflight"),
        "stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("network_policies includes disabled"),
        "stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("approval_modes includes never"),
        "stderr:\n{stderr}"
    );
    assert!(
        !stderr.contains("failed to start CAIC executable"),
        "stderr:\n{stderr}"
    );
    assert!(
        !marker.exists(),
        "intent review must not invoke CAIC run after a failed capabilities preflight"
    );
}

// A stand-in CAIC runner that answers `capabilities` with a backend which does NOT
// satisfy the review preflight, and records a marker if `run` is ever reached. It is
// a stub for the preflight negotiation only — the assertions it supports are about
// what `intent` refuses to do, never about what the real runner would return.
const FAKE_CODING_AGENT_UNSUPPORTED_DEFAULT: &str = r#"#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  capabilities)
    [ "${2:-}" = "--json" ] || exit 2
    cat <<'JSON'
{
  "schema_version": "coding_agent.capabilities.v1",
  "default_backend": "opencode",
  "backends": [
    {
      "id": "opencode",
      "modes": ["review"],
      "permissions": ["read-only"],
      "config_policies": ["isolated"],
      "network_policies": ["provider-default"],
      "approval_modes": ["on-request"],
      "output_formats": ["json"],
      "schema_output": false
    }
  ]
}
JSON
    ;;
  run)
    touch "${FAKE_CAIC_RUN_MARKER:?}"
    printf 'run should not be reached\n' >&2
    exit 99
    ;;
  *)
    printf 'unexpected fake coding-agent command: %s\n' "${1:-}" >&2
    exit 2
    ;;
esac
"#;
