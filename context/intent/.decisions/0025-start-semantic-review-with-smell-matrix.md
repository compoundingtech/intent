# Start semantic review with smell matrix fixtures

Status: accepted

## Context

Semantic-review fixtures now live under
`intent/15-evaluation/semantic-review/`. The next question is which first
fixtures should prove the baked review prompt and schema.

The fixture set can start as a small smell matrix, a richer end-to-end scenario,
a paired good/bad corpus for every artifact kind, or no fixtures until
`intent review` is implemented.

## Evidence and Argument

The baked prompt delegates review judgment to the root Intent contract review-smell catalog.
The first fixtures should therefore exercise the highest-signal smells with
one semantic concern per fixture. Narrow fixtures make prompt regressions easier
to diagnose than one mixed scenario, and they avoid building a broad corpus
before `intent review` has production command wiring.

The selected smells cover core artifact-boundary drift: vision containing
mechanism, requirements containing mechanism, specs containing decision
rationale, stale open questions, and stale deltas.

## Options

| Option | Tradeoffs |
| --- | --- |
| Minimal review-smell matrix | Fast to review, directly maps to the prompt rubric, and isolates regressions, but does not prove mixed real-world scenarios. |
| One richer end-to-end scenario | Closer to real use, but harder to diagnose which prompt behavior regressed. |
| Good/bad paired corpus per artifact kind | Strong coverage, but more initial maintenance before review wiring is production-ready. |
| No fixtures yet | Avoids fixture churn, but leaves the prompt and schema unproven. |

## Decision

The first semantic-review fixture set is a minimal review-smell matrix:

- `vision-mechanism`;
- `requirements-mechanism`;
- `spec-rationale`;
- `stale-open-question`;
- `stale-delta`.

Each fixture contains one primary expected semantic finding and uses the full
`axe.intent.review.v1` result shape.

## Consequences

- Prompt tuning starts with isolated, easy-to-debug regressions.
- Richer mixed scenarios and good/bad paired corpora remain future expansions.
- Fixture additions should continue to map to explicit review-smell or
  file-kind contracts rather than inventing fixture-local review rules.
