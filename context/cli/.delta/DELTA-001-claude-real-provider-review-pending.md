# DELTA-001: Claude real-provider review report is pending

Status: open

## Divergence

`intent review` is implemented and covered by fake-provider tests. Production
readiness is tracked per backend: the stock `codex` path has produced a
semantic review report for a small Intent subsystem, while the stock `claude` path
still lacks an authenticated isolated real-provider report.

## Intent

- [spec.md](../spec.md) defines `intent review` backend readiness as
  backend-scoped.
- [0001-use-per-backend-review-readiness.md](../.decisions/0001-use-per-backend-review-readiness.md)
  records the readiness policy.
- [intent/16-enforcement](../../../intent/16-enforcement/) owns the baked
  semantic-review prompt and result schema that provider reports must satisfy.

## Implementation

- CAIC fake-provider tests cover the Claude command-mapping regressions that
  previously caused timeout-shaped failures.
- The current isolated real-provider blocker is authenticated Claude execution:
  this worktree's `--bare` Claude probe fails immediately with
  `Not logged in · Please run /login`.

## Evidence

- Improved CAIC provider-failure diagnostics exposed Codex native schema
  compatibility issues in the baked `axe.intent.review.v1` schema.
- After adding explicit string types and requiring `gate`,
  `intent review context/intent/16-enforcement --backend codex
  --timeout-seconds 180 --report ...` succeeded and wrote an
  `axe.intent.review.v1` report.
- `intent review context/intent/16-enforcement --backend claude
  --timeout-seconds 180 --report ...` failed with CAIC `timeout` before writing
  a report.
- Follow-up wrapper inspection found that the Claude command path passed
  `MultiEdit` in `--disallowedTools`. The installed Claude CLI rejects unknown
  deny rules, so CAIC now uses only the currently accepted mutating tool names
  (`Edit`, `Write`, `NotebookEdit`, `Bash`) and pins `--model sonnet` to avoid
  provider-default Opus latency/cost surprises.
- The same inspection found the timeout root cause: CAIC placed Claude's
  variadic `--add-dir <directories...>` option immediately before the
  positional prompt. Claude consumed the prompt as another directory argument,
  leaving the review invocation without prompt input. CAIC now emits `--add-dir`
  before later scalar flags so the final positional prompt remains unambiguous.
- In this worktree, isolated `--bare` Claude probes fail immediately with
  `Not logged in · Please run /login` and zero token spend because no
  `ANTHROPIC_API_KEY` is present. The wrapper timeout class is covered by fake
  provider tests; the full real Claude report still requires API-key-backed
  isolated auth.
- The Intent/CLI decision is to treat review production readiness per backend
  rather than requiring every stock backend to pass before a proven backend can
  be used.

## Required Reconciliation

- `intent review --backend claude` should complete a bounded manual
  real-provider eval against a small existing Intent subsystem using isolated auth
  and write an `axe.intent.review.v1` report.
- The successful Claude eval evidence should be captured under the relevant
  `.experiments/` directory before this delta is removed.
- This delta does not block `codex` readiness. It blocks claiming Claude
  readiness for this command.

## Direction

update implementation

## Resolution Signal

Delete this delta when a bounded manual `claude` run writes a schema-valid
`axe.intent.review.v1` report for a small existing Intent subsystem, and that report
is captured under the owning `.experiments/` directory.
