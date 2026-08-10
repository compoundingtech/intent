# Intent Evaluation — Spec

This document specifies isolated evaluation runs for Intent and Intent skills. It
builds on [requirements.md](./requirements.md).

## Status

Draft.

## Scenario Shape

An eval scenario should be small enough to run quickly but rich enough to force
artifact routing decisions:

```text
/tmp/intent-eval-<id>/
  context/<scenario>/
    vision.md
    requirements.md
    spec.md
    ontology.md
    open-questions.md
    roadmap.md
    .decisions/
    .experiments/
    .reference/
    .delta/
```

The scenario should include at least:

- a CLI, config, API, UI, or event surface;
- an external system or platform constraint with reference material;
- a data model or state/lifecycle shape;
- a verification loop or test architecture;
- one consequential decision requiring options, evidence, tradeoffs, and choice
  rationale;
- one fuzzy or overloaded term that exercises ontology handling.

## Eval Procedure

1. Read `context/intent/` and `nixpkgs/ai/skills/grill-intent.md`.
2. Create the scenario in a temporary directory outside tracked repo files.
3. Apply the `grill-intent` procedure as if shaping the scenario with a user.
4. Produce the miniature Intent tree.
5. Review the tree with the review-smell catalog.
6. Report evidence-backed findings and classify each gap by owning artifact.

## Semantic Review Fixtures

Semantic-review fixtures live under
[semantic-review/](./semantic-review/). They tune and regression-test the
baked review prompt and result schema owned by
[16-enforcement](../16-enforcement/spec.md).

Tracked fixtures are canonical inputs and expected outcomes, not eval run
output. A semantic-review eval copies or materializes a fixture into an
isolated temporary scenario before invoking `intent review` through the Coding
Agent Invocation Contract. This preserves the isolated-eval requirement while
keeping prompt-quality examples reviewable in the Intent tree.

Use semantic-review fixtures for:

- known-good Intent examples that should produce no findings;
- known-bad examples for review-smell coverage;
- edge cases where deterministic diagnostics should not be repeated as semantic
  findings;
- cases that protect against prompt drift, schema drift, or overconfident
  findings when supplied context is insufficient.

Use `context/coding-agents/14-axe/12-intent/.experiments/` for command-integration
prototypes that validate Axe or CAIC wiring rather than semantic-review quality.

Fixture shape is validated before real semantic review runs. The first validator
is a deterministic Nix check that verifies fixture manifests, JSON syntax,
prompt/schema references, fixture-relative expected artifact paths, and optional
generated-diagnostics markers. `expected-review.json` is validated against the
enforcement-owned result schema itself rather than a reimplementation of it, so
the free gate cannot drift from the contract a real provider is held to. The
check does not call a coding agent and does not claim to prove review quality.

Real semantic-review eval runners should assert minimum finding identity rather
than exact review text: each expected finding must match by rule, severity,
artifact, and owner. Summary, evidence wording, and suggested-fix wording may
change as long as the finding remains grounded and actionable.

Eval runs that spend real provider/model tokens are always manual. They must not
run from CI, ordinary Nix checks, pre-commit hooks, scheduled jobs, or any other
automatic gate. Automated checks may validate fixture shape and fake-provider
wiring, but a human or coding agent must explicitly start each real-token eval
run with a bounded target set. Manual review commands do not need a second
token-spend confirmation flag; they must instead fail before provider invocation
when they detect known automated environments.

## Report Shape

```markdown
# Intent Eval Report

Scenario: <name>
Artifacts: <temp path>

## What Worked

## Gaps

| Finding | Evidence | Owner | Recommendation |
| --- | --- | --- | --- |

## Suggested Intent Updates
```

Temporary eval artifacts are evidence, not normative Intent. Durable changes are
applied to the real Intent tree only after review.

When an eval creates a valid example and exposes a root Intent contract gap, preserve both:
the example demonstrates what worked, while the report classifies the gap by
owning artifact and proposes the minimal durable Intent update.
