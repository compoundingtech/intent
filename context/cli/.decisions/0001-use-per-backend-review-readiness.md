# Use per-backend review readiness

Status: accepted

## Context

`axe vrs review` can run through multiple stock CAIC backends. Manual
real-provider evals showed uneven provider state: `codex` produced a
schema-valid VRS review report for `context/vrs/16-enforcement`, while `claude`
still needs an authenticated isolated real-provider report after fixing its
wrapper-level command mapping.

The design question was whether one provider gap should keep the whole `axe vrs
review` surface marked not production-ready.

## Evidence and Argument

CAIC already exposes explicit backend ids and reports the effective backend in
result metadata. `axe vrs review` now preflights the selected backend through
`$CODING_AGENT capabilities --json` before provider invocation, so readiness can
be evaluated against the same contract for each backend.

The Codex path has real-provider evidence. The Claude path has token-free fake
regression coverage for the discovered command-mapping bugs, but lacks isolated
auth evidence in this worktree. Treating readiness globally would block a proven
backend on an unrelated provider authentication/eval gap.

## Options

| Option | Tradeoffs |
| --- | --- |
| Per-backend readiness | Lets proven backends be used while preserving explicit gaps for others; requires docs and deltas to name backend scope clearly. |
| All stock backends must pass | Strongest portability bar, but one provider-specific auth or runtime issue blocks every backend. |
| Keep the feature experimental until every backend passes | Conservative, but underuses successful real-provider and fake-provider evidence. |

## Decision

`axe vrs review` production readiness is backend-scoped.

A backend is ready when:

- its CAIC capability preflight satisfies the review contract;
- token-free fake-provider tests cover its provider-specific command mapping;
- at least one bounded manual real-provider run against a small existing VRS
  subsystem writes a schema-valid `axe.vrs.review.v1` report.

## Consequences

- The proven `codex` path can be treated independently from the pending
  `claude` real-provider report.
- Deltas and experiments must name the backend whose readiness is missing.
- Adding future backends does not regress existing ready backends as long as the
  default and selected-backend behavior remains explicit.
