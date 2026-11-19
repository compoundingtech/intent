use parking_lot::Mutex;
use std::{fs, path::PathBuf, process::Output, sync::Arc};

use axum::{
    body::to_bytes,
    extract::{Request, State},
    response::{IntoResponse, Response},
    Json, Router,
};
use serde_json::{json, Value};
use tokio::{net::TcpListener, process::Command, task::JoinHandle};

#[derive(Clone)]
struct ReplayState {
    probability: f64,
    result: Value,
    requests: Arc<Mutex<Vec<Value>>>,
}

struct Replay {
    origin: String,
    state: ReplayState,
    task: JoinHandle<()>,
}

impl Drop for Replay {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl Replay {
    async fn start(probability: f64, result: Value) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let state = ReplayState {
            probability,
            result,
            requests: Arc::default(),
        };
        let router = Router::new().fallback(reply).with_state(state.clone());
        let task = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            origin,
            state,
            task,
        }
    }
}

async fn reply(State(state): State<ReplayState>, request: Request) -> Response {
    let path = request.uri().path().to_owned();
    let body: Value = serde_json::from_slice(
        &to_bytes(request.into_body(), 4 * 1024 * 1024)
            .await
            .unwrap(),
    )
    .unwrap();
    state
        .requests
        .lock()
        .push(json!({"path": path, "body": body}));
    if path == "/v1/systemone" {
        let answers: serde_json::Map<String, Value> = body["questions"].as_object().unwrap().keys().map(|id| (
            id.clone(), json!({"type": "noul", "noul": if id == "smell-01" { state.probability } else { 0.1 }}),
        )).collect();
        Json(
            json!({"model": "provider/decision-reported", "answers": answers, "usage": {"input_tokens": 20, "output_tokens": 5, "total_tokens": 25}}),
        ).into_response()
    } else {
        assert_eq!(path, "/v1/chat/completions");
        if let Some(status) = state.result["replay_http_status"].as_u64() {
            let status = axum::http::StatusCode::from_u16(status as u16).unwrap();
            let response = state
                .result
                .get("replay_http_body")
                .cloned()
                .unwrap_or_else(|| json!({"error": "replay failure"}));
            return match response {
                Value::String(body) => (status, body).into_response(),
                body => (status, Json(body)).into_response(),
            };
        }
        if state.result["replay_delay"] == true {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
        Json(
            json!({"id": "replay", "object": "chat.completion", "created": 0, "model": "provider/grounding",
            "choices": [{"index": 0, "finish_reason": "stop", "message": {"role": "assistant", "content": state.result["replay_raw_content"].as_str().map(str::to_owned).unwrap_or_else(|| state.result.to_string())}}],
            "usage": {"prompt_tokens": 30, "completion_tokens": 10, "total_tokens": 40}}),
        ).into_response()
    }
}

struct Harness {
    temp: tempfile::TempDir,
    root: PathBuf,
    report: PathBuf,
}

impl Harness {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("context/intent");
        fs::create_dir_all(root.join("16-enforcement")).unwrap();
        fs::create_dir_all(root.join("13-review-smells")).unwrap();
        fs::write(temp.path().join(".git"), "fixture workspace boundary").unwrap();
        fs::write(
            root.join("vision.md"),
            "# Vision\nImplement this with ExampleTool.\n",
        )
        .unwrap();
        fs::write(
            root.join("requirements.md"),
            "# Requirements\nA user can review contracts.\n",
        )
        .unwrap();
        fs::write(
            root.join("16-enforcement/review-prompt.md"),
            "Use only supplied artifacts; return axe.intent.review.v1.",
        )
        .unwrap();
        fs::write(
            root.join("16-enforcement/review-result.schema.json"),
            include_str!("../../../context/intent/16-enforcement/review-result.schema.json"),
        )
        .unwrap();
        fs::write(
            root.join("13-review-smells/spec.md"),
            include_str!("../../../context/intent/13-review-smells/spec.md"),
        )
        .unwrap();
        let report = temp.path().join("result.json");
        Self { temp, root, report }
    }

    async fn run(&self, replay: &Replay, args: &[&str], opt_in: Option<&str>) -> Output {
        self.run_at(replay, &self.root, "context/intent/vision.md", args, opt_in)
            .await
    }

    async fn run_at(
        &self,
        replay: &Replay,
        root: &std::path::Path,
        target: &str,
        args: &[&str],
        opt_in: Option<&str>,
    ) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_intent"));
        command
            .arg("review")
            .arg(root)
            .args(["--backend", "gateway", "--model", "provider/grounding"])
            .args(["--file", target, "--report"])
            .arg(&self.report)
            .args(args)
            .env("CI", "true")
            .env("AI_GATEWAY_URL", &replay.origin)
            .env_remove("AI_GATEWAY_TOKEN")
            .env_remove("INTENT_REVIEW_ALLOW_CI")
            .env_remove("INTENT_REVIEW_MODEL")
            .current_dir(self.temp.path());
        if let Some(value) = opt_in {
            command.env("INTENT_REVIEW_ALLOW_CI", value);
        }
        command.output().await.unwrap()
    }

    fn usage(&self) -> Value {
        serde_json::from_slice(&fs::read(self.report.with_extension("json.usage.json")).unwrap())
            .unwrap()
    }
}

/// The corpus-schema result the backend emits.
fn finding_result() -> Value {
    json!({"schema_version": "axe.intent.review.v1", "summary": "Vision contains mechanism.", "findings": [{
        "rule": "INTENT.REVIEW.vision-mechanism", "severity": "warning", "gate": "review", "artifact": "context/intent/vision.md",
        "owner": "context/intent/spec.md", "evidence": "Implement this with ExampleTool.", "suggested_fix": "Move implementation to spec.md."
    }]})
}

/// The grounding response: the emitted result plus each finding's smell id.
fn grounded(smell: &str) -> Value {
    let mut result = finding_result();
    result["findings"][0]["smell"] = json!(smell);
    result
}

#[tokio::test]
async fn gateway_replays_triage_and_one_grounding_call_with_shared_packet() {
    let h = Harness::new();
    let replay = Replay::start(0.6, grounded("smell-01")).await;
    let output = h.run(&replay, &["--allow-ci"], None).await;
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    let result: Value = serde_json::from_slice(&fs::read(&h.report).unwrap()).unwrap();
    assert_eq!(result, finding_result());
    let requests = replay.state.requests.lock();
    assert_eq!(requests.len(), 2);
    let decision = &requests[0]["body"];
    assert_eq!(decision["model"], "openrouter/~typesafe/jev-latest");
    assert_eq!(decision["questions"].as_object().unwrap().len(), 17);
    assert!(decision["questions"]
        .as_object()
        .unwrap()
        .values()
        .all(|q| q["type"] == "noul"));
    assert_eq!(decision["state"]["artifacts"].as_object().unwrap().len(), 1);
    assert!(decision["state"]["artifacts"]["context/intent/vision.md"]
        .as_str()
        .unwrap()
        .contains("ExampleTool"));
    assert!(decision["state"]["deterministic_diagnostics"]["diagnostics"].is_array());
    assert!(decision["state"]["prompt"].is_string());
    let grounding = &requests[1]["body"];
    assert_eq!(grounding["response_format"]["type"], "json_schema");
    let schema = &grounding["response_format"]["json_schema"]["schema"];
    assert!(schema.to_string().contains("axe.intent.review.v1"));
    assert_eq!(
        schema["properties"]["findings"]["items"]["properties"]["smell"]["enum"],
        json!(["smell-01"])
    );
    let packet: Value =
        serde_json::from_str(grounding["messages"][1]["content"].as_str().unwrap()).unwrap();
    assert_eq!(
        packet["flagged_smells"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        ["smell-01"]
    );
    assert!(packet["packet"].get("rubric").is_none());
    assert!(packet["packet"]["artifacts"].is_object());
    let usage = h.usage();
    assert_eq!(usage["outcome"], "ok");
    assert_eq!(usage["smells"]["smell-01"]["probability"], 0.6);
    assert_eq!(usage["decision"]["usage"]["total"], 25);
    assert_eq!(
        usage["decision"]["reported_model"],
        "provider/decision-reported"
    );
    assert_eq!(usage["grounding"]["usage"]["total"], 40);
}

#[tokio::test]
async fn gateway_abstains_below_threshold_without_grounding() {
    let h = Harness::new();
    let replay = Replay::start(0.599, grounded("smell-01")).await;
    let output = h.run(&replay, &[], Some("gateway")).await;
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(replay.state.requests.lock().len(), 1);
    let result: Value = serde_json::from_slice(&fs::read(&h.report).unwrap()).unwrap();
    assert_eq!(result["schema_version"], "axe.intent.review.v1");
    assert_eq!(result["findings"], json!([]));
    assert_eq!(h.usage()["grounding"]["called"], false);
    assert_eq!(h.usage()["grounding"]["usage"], Value::Null);
}

#[tokio::test]
async fn gateway_strict_findings_fail_but_advisory_findings_do_not() {
    let h = Harness::new();
    let replay = Replay::start(0.9, grounded("smell-01")).await;
    assert_eq!(
        h.run(&replay, &["--allow-ci", "--strict"], None)
            .await
            .status
            .code(),
        Some(1)
    );
    assert!(h.report.exists());
}

#[tokio::test]
async fn gateway_rejects_invalid_schema_unsupplied_artifacts_and_unflagged_smells() {
    for invalid in [
        json!({"schema_version": "wrong", "summary": "Invalid", "findings": []}),
        {
            let mut result = grounded("smell-01");
            result["findings"][0]["artifact"] = json!("context/intent/requirements.md");
            result
        },
        finding_result(),
        grounded("smell-02"),
    ] {
        let h = Harness::new();
        let replay = Replay::start(0.9, invalid.clone()).await;
        let output = h.run(&replay, &["--allow-ci"], None).await;
        assert_eq!(output.status.code(), Some(2), "{invalid}");
        assert!(!h.report.exists(), "{invalid}");
        let usage = h.usage();
        assert_eq!(usage["outcome"], "error");
        assert_eq!(usage["error_class"], "schema");
        assert_eq!(usage["decision"]["usage"]["total"], 25);
        assert_eq!(usage["grounding"]["called"], true);
        assert_eq!(usage["grounding"]["usage"]["total"], 40, "{invalid}");
    }
}

#[tokio::test]
async fn gateway_ci_requires_exact_opt_in_and_never_invokes_caic_preflight() {
    let h = Harness::new();
    let replay = Replay::start(0.1, grounded("smell-01")).await;
    for opt_in in [None, Some("true"), Some("claude")] {
        let output = h.run(&replay, &[], opt_in).await;
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("automated context (CI)"));
    }
    assert!(replay.state.requests.lock().is_empty());
    let output = h
        .run(
            &replay,
            &["--allow-ci", "--coding-agent", "/nonexistent-caic"],
            None,
        )
        .await;
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[tokio::test]
async fn caic_ci_stays_refused_even_with_gateway_opt_in() {
    let h = Harness::new();
    for backend in ["claude", "opencode"] {
        let output = Command::new(env!("CARGO_BIN_EXE_intent"))
            .arg("review")
            .arg(&h.root)
            .args([
                "--backend",
                backend,
                "--allow-ci",
                "--coding-agent",
                "/nonexistent-caic",
            ])
            .env("CI", "true")
            .env("INTENT_REVIEW_ALLOW_CI", "gateway")
            .output()
            .await
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("automated context (CI)"));
    }
}

#[tokio::test]
async fn gateway_rejects_invalid_threshold_and_target_before_network() {
    let h = Harness::new();
    let replay = Replay::start(0.9, grounded("smell-01")).await;
    for args in [
        vec!["--allow-ci", "--probability-threshold", "NaN"],
        vec!["--allow-ci", "--file", "../outside.md"],
    ] {
        assert_eq!(h.run(&replay, &args, None).await.status.code(), Some(2));
    }
    assert!(replay.state.requests.lock().is_empty());
    assert!(!h.report.with_extension("json.usage.json").exists());
}

#[tokio::test]
async fn gateway_preserves_decision_usage_on_auth_failure_and_grounding_timeout() {
    for (response, args, class) in [
        (
            json!({"replay_http_status": 401}),
            vec!["--allow-ci"],
            "auth",
        ),
        (
            json!({"replay_delay": true}),
            vec!["--allow-ci", "--timeout-seconds", "1"],
            "timeout",
        ),
    ] {
        let h = Harness::new();
        let replay = Replay::start(0.9, response).await;
        let output = h.run(&replay, &args, None).await;
        assert_eq!(output.status.code(), Some(2));
        let usage = h.usage();
        assert_eq!(usage["outcome"], "error");
        assert_eq!(usage["error_class"], class);
        assert_eq!(usage["decision"]["usage"]["total"], 25);
        assert_eq!(usage["grounding"]["called"], true);
        assert_eq!(usage["grounding"]["usage"], Value::Null);
        assert!(!h.report.exists());
    }
}

#[tokio::test]
async fn gateway_uses_explicit_asset_corpus_with_sibling_target_tree() {
    let h = Harness::new();
    let target = "context/other/vision.md";
    let path = h.temp.path().join(target);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, "# Vision\nImplement this with ExampleTool.\n").unwrap();
    let mut response = grounded("smell-01");
    response["findings"][0]["artifact"] = json!(target);
    let replay = Replay::start(0.9, response).await;
    let output = h
        .run_at(
            &replay,
            h.root.parent().unwrap(),
            target,
            &["--allow-ci", "--corpus", "context/intent"],
            None,
        )
        .await;
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = replay.state.requests.lock();
    let state = &requests[0]["body"]["state"];
    assert!(state["rubric"]
        .as_str()
        .unwrap()
        .contains("## Smell Catalog"));
    assert_eq!(state["artifacts"].as_object().unwrap().len(), 1);
    assert!(state["artifacts"][target]
        .as_str()
        .unwrap()
        .contains("ExampleTool"));
    assert_eq!(
        state["prompt"],
        "Use only supplied artifacts; return axe.intent.review.v1."
    );
    let result: Value = serde_json::from_slice(&fs::read(&h.report).unwrap()).unwrap();
    assert_eq!(result["schema_version"], "axe.intent.review.v1");
    assert_eq!(result["findings"][0]["artifact"], target);
}

#[tokio::test]
async fn explicit_corpus_does_not_allow_a_file_outside_the_target_root() {
    let h = Harness::new();
    fs::write(h.temp.path().join("outside.md"), "# Outside\n").unwrap();
    let replay = Replay::start(0.9, grounded("smell-01")).await;
    let output = h
        .run_at(
            &replay,
            h.root.parent().unwrap(),
            "outside.md",
            &["--allow-ci", "--corpus", "context/intent"],
            None,
        )
        .await;
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid review target"));
    assert!(replay.state.requests.lock().is_empty());
    assert!(!h.report.with_extension("json.usage.json").exists());
}

#[tokio::test]
async fn gateway_preserves_billed_usage_when_decision_validation_rejects_answer() {
    let h = Harness::new();
    let replay = Replay::start(1.1, grounded("smell-01")).await;
    let output = h.run(&replay, &["--allow-ci"], None).await;
    assert_eq!(output.status.code(), Some(2));
    let usage = h.usage();
    assert_eq!(usage["outcome"], "error");
    assert_eq!(usage["error_class"], "schema");
    assert_eq!(usage["decision"]["usage"]["total"], 25);
    assert_eq!(usage["grounding"]["called"], false);
    assert_eq!(replay.state.requests.lock().len(), 1);
    assert!(!h.report.exists());
}

impl Harness {
    fn fixture_root(&self) -> PathBuf {
        let root = self.root.join("15-evaluation/semantic-review");
        let fixture = root.join("vision-mechanism");
        fs::create_dir_all(fixture.join("input")).unwrap();
        fs::write(
            fixture.join("input/vision.md"),
            "# Vision\nImplement this with ExampleTool.\n",
        )
        .unwrap();
        fs::write(
            fixture.join("fixture.json"),
            json!({
                "assertion_mode": "minimum_findings",
                "prompt_ref": "../../../16-enforcement/review-prompt.md",
                "schema_ref": "../../../16-enforcement/review-result.schema.json"
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            fixture.join("assertions.json"),
            json!({"minimum_findings": [{
                "rule": "INTENT.REVIEW.vision-mechanism", "severity": "warning",
                "artifact": "input/vision.md", "owner": "input/spec.md"
            }]})
            .to_string(),
        )
        .unwrap();
        root
    }

    fn fixture_command(&self, replay: &Replay) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_intent"));
        command
            .arg("review-fixtures")
            .arg(self.fixture_root())
            .args([
                "--backend",
                "gateway",
                "--model",
                "provider/grounding",
                "--decision-model",
                "provider/decision",
                "--coding-agent",
                "/nonexistent-caic",
            ])
            .arg("--report")
            .arg(&self.report)
            .env("CI", "true")
            .env("AI_GATEWAY_URL", &replay.origin)
            .env_remove("AI_GATEWAY_TOKEN")
            .env_remove("INTENT_REVIEW_ALLOW_CI")
            .env_remove("INTENT_REVIEW_MODEL")
            .current_dir(self.temp.path());
        command
    }

    async fn run_fixtures(&self, replay: &Replay, args: &[&str]) -> Output {
        self.fixture_command(replay)
            .args(args)
            .output()
            .await
            .unwrap()
    }

    fn fixture_report(&self) -> Value {
        serde_json::from_slice(&fs::read(&self.report).unwrap()).unwrap()
    }
}

fn fixture_grounded() -> Value {
    let mut result = grounded("smell-01");
    result["findings"][0]["artifact"] = json!("input/vision.md");
    result["findings"][0]["owner"] = json!("input/spec.md");
    result
}

#[tokio::test]
async fn gateway_fixtures_pass_all_minimum_findings_with_isolated_packet_and_usage() {
    let h = Harness::new();
    let replay = Replay::start(0.9, fixture_grounded()).await;
    let output = h
        .run_fixtures(&replay, &["--allow-ci", "--corpus", "context/intent"])
        .await;
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report = h.fixture_report();
    assert_eq!(report["passed"], 1);
    assert_eq!(
        report["fixtures"][0]["matched"].as_array().unwrap().len(),
        1
    );
    let requests = replay.state.requests.lock();
    assert_eq!(requests[0]["body"]["model"], "provider/decision");
    assert_eq!(
        requests[0]["body"]["state"]["artifacts"]
            .as_object()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        ["input/vision.md"]
    );
    let usage: Value = serde_json::from_slice(
        &fs::read(
            h.temp
                .path()
                .join("result.json.usage/vision-mechanism.usage.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(usage["outcome"], "ok");
    assert_eq!(usage["grounding"]["usage"]["total"], 40);
    assert_eq!(
        fs::read_to_string(
            h.root
                .join("15-evaluation/semantic-review/vision-mechanism/input/vision.md")
        )
        .unwrap(),
        "# Vision\nImplement this with ExampleTool.\n"
    );
}

#[tokio::test]
async fn gateway_fixtures_fail_missing_minimum_finding() {
    let h = Harness::new();
    let replay = Replay::start(0.1, fixture_grounded()).await;
    let output = h.run_fixtures(&replay, &["--allow-ci"]).await;
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(h.fixture_report()["failed"], 1);
    assert_eq!(
        h.fixture_report()["fixtures"][0]["missing"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn gateway_fixtures_grounding_parse_error_is_errored() {
    let h = Harness::new();
    let replay = Replay::start(0.9, json!({"replay_raw_content": "{not json"})).await;
    let output = h.run_fixtures(&replay, &["--allow-ci"]).await;
    assert_eq!(output.status.code(), Some(2));
    let report = h.fixture_report();
    assert_eq!(report["errored"], 1);
    assert_eq!(report["fixtures"][0]["status"], "errored");
    assert!(report["fixtures"][0]["reason"]
        .as_str()
        .unwrap()
        .contains("validation"));
}

#[tokio::test]
async fn gateway_fixtures_ci_requires_opt_in_before_network_or_caic() {
    let h = Harness::new();
    let replay = Replay::start(0.9, fixture_grounded()).await;
    let output = h.run_fixtures(&replay, &[]).await;
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("automated context (CI)"));
    assert!(replay.state.requests.lock().is_empty());
}

#[tokio::test]
async fn caic_fixtures_ci_rejected_even_with_gateway_opt_in() {
    let h = Harness::new();
    let replay = Replay::start(0.9, fixture_grounded()).await;
    let output = Command::new(env!("CARGO_BIN_EXE_intent"))
        .arg("review-fixtures")
        .arg(h.fixture_root())
        .args([
            "--allow-ci",
            "--backend",
            "claude",
            "--coding-agent",
            "/nonexistent-caic",
        ])
        .env("CI", "true")
        .env("INTENT_REVIEW_ALLOW_CI", "gateway")
        .output()
        .await
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("automated context (CI)"));
    assert!(replay.state.requests.lock().is_empty());
}

#[tokio::test]
async fn gateway_fixtures_http_failure_and_timeout_preserve_usage_and_continue() {
    for (response, args, class) in [
        (
            json!({"replay_http_status": 502}),
            vec!["--allow-ci", "--usage-report", "usage"],
            "http",
        ),
        (
            json!({"replay_delay": true}),
            vec![
                "--allow-ci",
                "--timeout-seconds",
                "1",
                "--usage-report",
                "usage",
            ],
            "timeout",
        ),
    ] {
        let h = Harness::new();
        let root = h.fixture_root();
        let second = root.join("second");
        fs::create_dir_all(second.join("input")).unwrap();
        for file in ["fixture.json", "assertions.json", "input/vision.md"] {
            fs::copy(root.join("vision-mechanism").join(file), second.join(file)).unwrap();
        }
        let replay = Replay::start(0.9, response).await;
        let output = h.run_fixtures(&replay, &args).await;
        assert_eq!(output.status.code(), Some(2));
        assert_eq!(h.fixture_report()["errored"], 2);
        assert_eq!(replay.state.requests.lock().len(), 4);
        for id in ["second", "vision-mechanism"] {
            let usage: Value = serde_json::from_slice(
                &fs::read(h.temp.path().join("usage").join(format!("{id}.usage.json"))).unwrap(),
            )
            .unwrap();
            assert_eq!(usage["outcome"], "error");
            assert_eq!(usage["error_class"], class);
            assert_eq!(usage["decision"]["usage"]["total"], 25);
        }
        if class == "http" {
            assert!(h.fixture_report()["fixtures"][0]["reason"]
                .as_str()
                .unwrap()
                .contains("http (status 502)"));
        }
    }
}

const SENTINEL_MODEL_TEXT: &str = "SENTINEL_MODEL_TEXT_DO_NOT_DISCLOSE";
const SENTINEL_CREDENTIAL: &str = "SENTINEL_UPSTREAM_CREDENTIAL";
const SENTINEL_GATEWAY_TOKEN: &str = "SENTINEL_GATEWAY_TOKEN";

#[tokio::test]
async fn gateway_fixture_http_errors_never_disclose_upstream_content() {
    let sensitive = format!("{SENTINEL_MODEL_TEXT} {SENTINEL_CREDENTIAL} {SENTINEL_GATEWAY_TOKEN}");
    for body in [
        json!({"error": {"message": sensitive, "type": sensitive}}),
        json!(sensitive),
    ] {
        let h = Harness::new();
        let replay = Replay::start(
            0.9,
            json!({
                "replay_http_status": 502, "replay_http_body": body,
            }),
        )
        .await;
        let output = h
            .fixture_command(&replay)
            .arg("--allow-ci")
            .env("AI_GATEWAY_TOKEN", SENTINEL_GATEWAY_TOKEN)
            .output()
            .await
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        let report_bytes = fs::read(&h.report).unwrap();
        let report: Value = serde_json::from_slice(&report_bytes).unwrap();
        let entry = &report["fixtures"][0];
        let grade = intent::FixtureGrade {
            id: entry["id"].as_str().unwrap().to_owned(),
            status: "errored",
            assertion_mode: None,
            matched: Vec::new(),
            missing: Vec::new(),
            reason: Some(entry["reason"].as_str().unwrap().to_owned()),
        };
        assert_eq!(
            grade.reason.as_deref(),
            Some("gateway review error: http (status 502)")
        );
        let debug = format!("{grade:?}");
        for surface in [
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&report_bytes),
            std::borrow::Cow::Borrowed(debug.as_str()),
        ] {
            for secret in [
                SENTINEL_MODEL_TEXT,
                SENTINEL_CREDENTIAL,
                SENTINEL_GATEWAY_TOKEN,
            ] {
                assert!(
                    !surface.contains(secret),
                    "sensitive response content escaped"
                );
            }
        }
    }
}

fn tree_snapshot(root: &std::path::Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    fn visit(
        root: &std::path::Path,
        directory: &std::path::Path,
        snapshot: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>,
    ) {
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let kind = entry.file_type().unwrap();
            let bytes = if kind.is_symlink() {
                fs::read_link(&path)
                    .unwrap()
                    .to_string_lossy()
                    .as_bytes()
                    .to_vec()
            } else if kind.is_file() {
                fs::read(&path).unwrap()
            } else {
                Vec::new()
            };
            snapshot.insert(path.strip_prefix(root).unwrap().to_path_buf(), bytes);
            if kind.is_dir() {
                visit(root, &path, snapshot);
            }
        }
    }
    let mut snapshot = std::collections::BTreeMap::new();
    visit(root, root, &mut snapshot);
    snapshot
}

#[tokio::test]
async fn gateway_fixture_report_usage_collision_rejected_without_calls_or_writes() {
    for report in [
        "usage/vision-mechanism.usage.json",
        "./usage/nested/../vision-mechanism.usage.json",
    ] {
        let mut h = Harness::new();
        h.report = PathBuf::from(report);
        let replay = Replay::start(0.9, fixture_grounded()).await;
        let mut command = h.fixture_command(&replay);
        command.args(["--allow-ci", "--usage-report", "usage"]);
        let before = tree_snapshot(h.temp.path());
        let output = command.output().await.unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("--report must differ"));
        assert!(replay.state.requests.lock().is_empty());
        assert_eq!(tree_snapshot(h.temp.path()), before);
    }
}

#[cfg(unix)]
#[tokio::test]
async fn gateway_fixture_symlinked_report_usage_collision_rejected_without_writes() {
    let mut h = Harness::new();
    h.report = PathBuf::from("alias/./nested/../vision-mechanism.usage.json");
    fs::create_dir_all(h.temp.path().join("usage/nested")).unwrap();
    fs::write(
        h.temp.path().join("usage/vision-mechanism.usage.json"),
        "preserve existing usage",
    )
    .unwrap();
    std::os::unix::fs::symlink("usage", h.temp.path().join("alias")).unwrap();
    let replay = Replay::start(0.9, fixture_grounded()).await;
    let mut command = h.fixture_command(&replay);
    command.args(["--allow-ci", "--usage-report", "usage"]);
    let before = tree_snapshot(h.temp.path());
    let output = command.output().await.unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("--report must differ"));
    assert!(replay.state.requests.lock().is_empty());
    assert_eq!(tree_snapshot(h.temp.path()), before);
}
