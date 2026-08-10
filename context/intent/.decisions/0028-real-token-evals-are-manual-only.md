# Keep real-token evals manual-only

Status: accepted

## Context

Semantic-review fixtures can be validated deterministically and later evaluated
through `$CODING_AGENT`. Running a real provider spends model tokens and may
consume quota, money, or rate-limit budget.

The question is whether real-token semantic-review evals may run automatically
from CI, Nix checks, scheduled jobs, or other gates, or whether they require an
explicit manual invocation.

## Evidence and Argument

Deterministic fixture validation already catches fixture shape, JSON, path, and
schema drift without model spend. Fake-provider CAIC checks can validate
invocation wiring without model spend. Those automated layers are enough for
normal local and CI safety.

Real-provider evals are useful for prompt quality, but they have an operational
cost and may vary with provider behavior. If they run automatically, routine CI
or local checks can spend tokens without a deliberate decision by the operator.
That violates the principle that token-spending evals should be intentional and
bounded.

## Options

| Option | Tradeoffs |
| --- | --- |
| Manual-only real-token evals | Prevents accidental token spend and keeps eval scope deliberate, but requires a human or agent to start review-quality evals explicitly. |
| CI-gated real-token evals | Strong automation signal, but risks surprise spend, flakiness, and quota pressure. |
| Scheduled real-token evals | Catches drift over time, but can spend tokens without task-local need. |
| Nix check real-token evals | Fits existing check workflow, but makes ordinary validation non-free and provider-dependent. |

## Decision

Real provider/model-token evals are manual-only.

They must not run from CI, ordinary Nix checks, pre-commit hooks, scheduled jobs,
or default automated validation. A human or coding agent must explicitly start
each real-token eval run and choose a bounded target set.

Automated checks may validate:

- fixture shape;
- JSON/schema conformance;
- deterministic checker output;
- fake-provider invocation wiring.

## Consequences

- Real semantic-review quality checks remain available but intentional.
- CI and Nix checks stay deterministic and token-free.
- Future `intent review` or `intent review-fixtures` commands must fail or
  refuse when invoked as automatic gates with real providers.
