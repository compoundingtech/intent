# CAIC Semantic Review Prototype

Date: 2026-06-18

Current status: superseded by the implemented `intent review` command, the
baked assets in `context/intent/16-enforcement/`, and the real-provider eval in
`context/intent/16-enforcement/.experiments/2026-06-19-real-provider-review-eval.md`.
Keep this document as historical prototype evidence, not as current remaining
work.

## Question

Can `intent review` use the Coding Agent Invocation Contract (CAIC) cleanly for
semantic review while keeping deterministic `intent` primitives as the first
production milestone?

## Method

The prototype used the implemented `coding-agent` binary with a fake Codex
provider. The temporary scenario contained:

- a small Intent node with `requirements.md` and `spec.md`;
- a generated `intent check` diagnostics artifact;
- a candidate baked review prompt;
- a task-specific JSON Schema for the review result.

The command shape was:

```text
$CODING_AGENT run \
  --cwd <repo-root> \
  --prompt-file <baked-intent-review-prompt> \
  --context-file normative:<intent-requirements> \
  --context-file normative:<intent-spec> \
  --context-file generated-diagnostics:<intent-check-json> \
  --mode review \
  --permission read-only \
  --approval never \
  --config-policy isolated \
  --network-policy disabled \
  --output-format json \
  --output-schema <intent-review-result-schema> \
  --backend codex
```

Candidate prompt shape:

```text
You are running `intent review`. Review only the supplied Intent artifacts and
diagnostics. Return JSON matching the schema. Do not propose file edits.
```

Candidate schema shape:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "required": ["schema_version", "summary", "findings"],
  "properties": {
    "schema_version": { "const": "axe.intent.review.v1" },
    "summary": { "type": "string" },
    "findings": {
      "type": "array",
      "items": {
        "type": "object",
        "required": [
          "rule",
          "severity",
          "artifact",
          "evidence",
          "suggested_fix"
        ],
        "properties": {
          "rule": { "type": "string" },
          "severity": { "enum": ["info", "warning", "error"] },
          "artifact": { "type": "string" },
          "evidence": { "type": "string" },
          "suggested_fix": { "type": "string" }
        },
        "additionalProperties": false
      }
    }
  },
  "additionalProperties": false
}
```

## Result

The CAIC route worked with the fake provider. The adapter:

- accepted workspace Intent files as `normative` context;
- accepted an outside diagnostics file only when its JSON had a generated
  artifact marker (`schema_version` or `producer`);
- invoked the backend in `review`, `read-only`, `approval never`,
  `config-policy isolated`, and `network-policy disabled`;
- validated the inner review result against the task schema;
- returned a normal `coding_agent.result.v1` envelope with the review result in
  `result`.

The successful inner result was:

```json
{
  "schema_version": "axe.intent.review.v1",
  "summary": "1 semantic concern found",
  "findings": [
    {
      "rule": "INTENT.REVIEW.spec.actionability",
      "severity": "warning",
      "artifact": "context/intent/spec.md",
      "evidence": "Spec uses vague language: should be good and flexible.",
      "suggested_fix": "Replace vague qualities with observable behavior and constraints."
    }
  ]
}
```

## Failure Modes Observed

When the generated diagnostics artifact was outside `--cwd` but lacked a
machine-readable generated marker, CAIC failed before provider execution:

```json
{
  "schema_version": "coding_agent.error.v1",
  "status": "failed",
  "exit_code": 2,
  "error": {
    "code": "invalid_invocation",
    "message": "generated context lacks schema_version or producer marker: <path>",
    "retryable": false
  }
}
```

This is desirable for `intent review`: the checker output must be a generated
artifact, not an arbitrary host file smuggled into review context.

## Tradeoffs

| Option | Result |
| --- | --- |
| Implement `intent review` now | CAIC is ready enough, but the deterministic checker, baked prompt file, and review schema are not yet production-shaped. This would couple review quality to unstable inputs. |
| Keep review experimental until primitives exist | Preserves the accepted implementation order and gives review a bounded diagnostics packet later. Delays semantic smells, but avoids noisy or ungrounded review output. |
| Add only hidden/prototype review command | Useful for local tuning, but risks an untracked command contract unless it is clearly excluded from public docs and CI. |

## Conclusion

Keep production `intent` focused on deterministic primitives first. In
parallel, create the baked semantic-review assets under Intent ownership:

- `context/intent/16-enforcement/review-prompt.md` or a sibling prompt artifact
  with the review rubric;
- `context/intent/16-enforcement/review-result.schema.json` for
  `axe.intent.review.v1`;
- evaluation fixtures under `context/intent/15-evaluation/` that tune the prompt
  against known good and bad Intent examples.

Once `intent check --json` emits stable diagnostics, `intent review` should
compose those diagnostics with the prompt/schema and call CAIC exactly through
the command route above.

## Intent Impact

This prototype informed the current Intent CLI review command, the baked
semantic-review prompt and schema owned by `context/intent/16-enforcement`, and the
manual-only real-provider eval policy. It is historical evidence only; current
readiness state lives in the spec, decisions, deltas, and real-provider evals.
