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
            return (
                axum::http::StatusCode::from_u16(status as u16).unwrap(),
                Json(json!({"error": "replay failure"})),
            )
                .into_response();
        }
        if state.result["replay_delay"] == true {
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        }
        Json(
            json!({"id": "replay", "object": "chat.completion", "created": 0, "model": "provider/grounding",
            "choices": [{"index": 0, "finish_reason": "stop", "message": {"role": "assistant", "content": state.result.to_string()}}],
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
        let mut command = Command::new(env!("CARGO_BIN_EXE_intent"));
        command
            .arg("review")
            .arg(&self.root)
            .args(["--backend", "gateway", "--model", "provider/grounding"])
            .args(["--file", "context/intent/vision.md", "--report"])
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
