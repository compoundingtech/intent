# Use minimum assertions for real semantic review evals

Status: accepted

## Context

Semantic-review fixtures can be checked structurally today. Later,
`axe vrs review` should run those fixtures through `$CODING_AGENT` and compare
the returned review result with expected behavior.

The assertion question is whether real semantic-review evals should require
exact `expected-review.json` matches, minimum finding assertions, a custom
hybrid assertion schema, or manual reports only.

## Evidence and Argument

Real provider output may improve summary wording, evidence phrasing, and
suggested-fix phrasing without changing the semantic behavior the fixture is
intended to protect. Exact full-result comparison would catch regressions, but
it would also make useful prompt improvements look like failures.

The stable part of a semantic-review fixture is finding identity: the review
should find the expected rule on the expected artifact with the expected
severity and owner. That is enough to catch prompt drift that misses or reroutes
the issue, while leaving wording flexible.

A custom hybrid schema could be more precise, but minimum finding assertions
provide the same first-order protection with less runner complexity.

## Options

| Option | Tradeoffs |
| --- | --- |
| Minimum finding assertions | Stable across wording improvements and catches missing or misrouted findings, but does not lock evidence text exactly. |
| Exact `expected-review.json` | Strongest regression lock, but brittle for real provider wording and prompt improvements. |
| Hybrid field/text assertions | More precise, but requires a custom assertion language before review quality is proven. |
| Manual report only | Low implementation cost, but does not protect prompt drift automatically. |

## Decision

Real semantic-review evals should use minimum finding assertions.

Each minimum assertion matches by:

- `rule`;
- `severity`;
- `artifact`;
- `owner`.

The runner should not require exact summary text, evidence wording, or
suggested-fix wording for real-provider runs.

## Consequences

- Fixtures can be useful for real LLM-backed evals without becoming brittle.
- `expected-review.json` remains a complete illustrative target and schema
  example.
- `assertions.json` becomes the stable comparison contract for real
  semantic-review eval runners.
