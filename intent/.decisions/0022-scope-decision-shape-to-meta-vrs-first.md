# Scope decision shape to meta-VRS first

Status: accepted

## Context

The decision-shape enforcement experiment found that the current meta-VRS
decision records already match the proposed strict mechanical shape, while the
wider `context/**/.decisions` corpus contains many older records that predate
the current decision contract.

The next question is whether initial enforcement should target only
`context/vrs/.decisions`, all context decisions in warning mode, changed
decisions only, or all decisions with an allowlist.

## Evidence and Argument

The owning contract for decision-record shape lives in the meta-VRS decision
subsystem. Applying the strict mechanical rule there first proves the rule on
the canonical examples and avoids turning legacy subsystem decisions into noisy
migration work before the checker exists.

Running the same rule across all context decisions immediately would expose real
debt, but the experiment showed the wider corpus is not ready. Changed-only
enforcement would avoid legacy churn, but it requires diff/base awareness that
is unnecessary for the first checker. An allowlist would make the rule
comprehensive, but would start the system with allowlist maintenance before the
rule has proven itself.

## Options

| Option | Tradeoffs |
| --- | --- |
| Meta-VRS only first | Cleanly proves strict shape on the owning contract and avoids legacy noise, but gives narrower coverage. |
| All context decisions warning-only | Makes migration debt visible, but can be noisy and may distract from proving the rule. |
| New or changed decisions only | Avoids legacy churn, but requires diff-aware enforcement. |
| All decisions with allowlist | Comprehensive, but creates allowlist maintenance before rule maturity. |

## Decision

Initial strict decision-shape enforcement applies only to
`context/vrs/.decisions/`.

Broader `context/**/.decisions/` checks may be introduced later as migration
diagnostics or scoped warnings after the checker has stable diagnostics and a
clear migration plan.

## Consequences

- The first decision-shape checker should use the meta-VRS decisions as the
  strict target corpus.
- Repo-wide decision-shape findings remain migration evidence, not merge
  blockers.
- The broader corpus can be migrated deliberately without weakening the strict
  shape contract for new meta-VRS decisions.
