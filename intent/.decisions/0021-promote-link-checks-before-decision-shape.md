# Promote link checks before decision shape

Status: accepted

## Context

VRS enforcement needs a first deterministic blocking rule after migration. The
candidates include local Markdown link existence, decision-record shape,
proposed decision cleanup, and open-question blocker checks.

The user selected local Markdown link existence first, with decision-record
shape as a follow-up only if experiments are promising.

## Evidence and Argument

Local Markdown link existence is a purely mechanical property in the current
commit. It has low ambiguity, gives immediate navigation value, and matches the
commit-scoped consistency model. False positives are also easy to calibrate
through explicit allowlists or fixture cases.

Decision-record shape is important, but the risk is that a mechanical checker
may reward decorative section stubs. A shape rule should therefore begin as an
experiment-backed warning: fixtures should show that it catches missing evidence,
missing alternatives, missing tradeoff discussion, or absent rationale without
pretending to verify decision quality semantically.

## Options

| Option | Tradeoffs |
| --- | --- |
| Make local Markdown link existence the first blocking rule | Low ambiguity and immediate value, but narrower than full VRS quality. |
| Make decision-record shape blocking immediately | Improves decision hygiene, but can incentivize decorative sections without evidence quality. |
| Block only on `.decisions/.proposed/` | Very crisp, but too narrow to validate the broader checker path. |
| Block on open-question blockers | Process-aligned, but too semantic for an early deterministic gate. |

## Decision

Make local Markdown link existence the first blocking deterministic rule after
migration. Treat decision-record shape as the second blocking candidate only
after experiments or fixtures show that the mechanically checkable subset is
useful and not mostly decorative.

## Consequences

- `axe vrs` enforcement should prioritize local link discovery, resolution,
  diagnostics, allowlist calibration, and strict-mode promotion first.
- Decision-record checks should start warning-only with explicit experiment
  criteria before becoming merge-blocking.
- Semantic decision quality remains a review concern even if mechanical shape
  eventually blocks missing required sections.
