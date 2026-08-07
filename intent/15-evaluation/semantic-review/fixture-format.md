# Semantic Review Fixture Format

Semantic-review fixtures tune `axe vrs review` and the baked prompt/schema
owned by [16-enforcement](../../16-enforcement/spec.md).

## Fixture Directory

Each fixture should live in a stable, descriptive directory:

```text
context/vrs/15-evaluation/semantic-review/<fixture-slug>/
  fixture.json
  input/
    context/<scenario>/
      requirements.md
      spec.md
      ...
  diagnostics/                    # optional
    axe-vrs-check.json
  expected-review.json
  assertions.json                 # optional
  notes.md
```

`fixture.json` is the machine-readable fixture manifest:

```json
{
  "schema_version": "vrs.semantic-review.fixture.v1",
  "id": "requirements-mechanism",
  "purpose": "Detect implementation mechanism in requirements.",
  "covers": ["VRS.REVIEW.requirements.mechanism"],
  "prompt_ref": "../../../16-enforcement/review-prompt.md",
  "schema_ref": "../../../16-enforcement/review-result.schema.json",
  "assertion_mode": "full | minimum",
  "known_limits": []
}
```

`input/` contains the tracked VRS artifacts copied into a temporary eval
workspace. Expected finding artifact paths should be fixture-relative, not
absolute paths and not paths from a specific temporary run.

`diagnostics/axe-vrs-check.json` is optional until the deterministic checker
output is relevant to the fixture; when present it must use a generated artifact
marker such as `schema_version` or `producer`.

`expected-review.json` contains a complete schema-valid `axe.vrs.review.v1`
result. Real-provider semantic evals should usually use `assertion_mode:
"minimum"` and assert only stable finding identity in `assertions.json`:

```json
{
  "schema_version": "vrs.semantic-review.assertions.v1",
  "minimum_findings": [
    {
      "rule": "VRS.REVIEW.requirements.mechanism",
      "severity": "warning",
      "artifact": "input/context/requirements-mechanism/requirements.md",
      "owner": "requirements.md"
    }
  ]
}
```

The runner should require each `minimum_findings` entry to appear in the review
result with matching `rule`, `severity`, `artifact`, and `owner`. It should not
require exact summary, evidence wording, or suggested-fix wording. Use
`assertion_mode: "full"` only for deterministic or fake-provider tests where
the entire result is intentionally stable.

`notes.md` explains the intent, evidence, and prompt behavior the fixture
protects.

## Rules

- Keep fixtures small enough for fast local review.
- Prefer one semantic question per fixture.
- Do not store eval run output in fixture directories.
- Copy fixtures to an isolated temporary workspace before running the eval.
- Do not let fixture cases define new review rules; rule semantics belong in
  the review-smell catalog, file-kind contracts, and enforcement prompt/schema.
- If a fixture proves a VRS contract gap, update the owning VRS artifact and
  preserve the fixture only when it continues to protect behavior.
- Use Axe VRS `.experiments/` for command plumbing evidence; use these fixtures
  for review quality.
