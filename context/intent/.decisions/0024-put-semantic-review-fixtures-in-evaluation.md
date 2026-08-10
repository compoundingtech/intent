# Put semantic review fixtures in evaluation

Status: accepted

## Context

After placing the baked semantic-review prompt and schema in
`intent/16-enforcement/`, Intent needs a home for the fixtures that tune and
regression-test those assets.

The ownership question is whether semantic-review fixtures belong with the Intent
evaluation subsystem, enforcement implementation evidence, CLI
experiments, or tool test fixtures.

## Evidence and Argument

The evaluation subsystem already owns isolated scenarios, evidence-backed
findings, and recommendation shape for Intent contract changes. Semantic-review
fixtures are not the prompt/schema themselves; they are examples used to learn
whether the prompt and schema produce useful review findings.

The existing evaluation requirements say eval runs must happen in isolated
temporary scenarios outside tracked project files. Tracked fixtures can coexist
with that rule if they are treated as canonical inputs and expected outcomes
that eval runners copy into a temporary workspace, not as run output.

Keeping fixtures under Axe would validate command plumbing well, but would make
review quality look tool-owned. Keeping them under enforcement would be close to
the prompt/schema, but would blur the line between enforcement contract and
evaluation evidence.

## Options

| Option | Tradeoffs |
| --- | --- |
| `intent/15-evaluation/semantic-review/` | Keeps prompt-quality evidence with the evaluation subsystem and preserves Intent ownership, but requires a clear distinction between tracked fixtures and temporary eval output. |
| `intent/16-enforcement/.experiments/` | Keeps fixtures near prompt/schema, but mixes reusable eval cases with enforcement implementation evidence. |
| `context/coding-agents/14-axe/12-intent/.experiments/` | Good for CLI and CAIC integration prototypes, but makes Axe look like the owner of semantic-review quality. |
| `flakes/axe/tests/fixtures/intent-review/` | Easy for tool tests, but separates semantic eval intent from the Intent source of truth. |

## Decision

Semantic-review eval fixtures live under
`intent/15-evaluation/semantic-review/`.

Tracked fixtures are canonical inputs and expected outcomes. Eval runs must copy
or materialize them into isolated temporary workspaces before invoking
`intent review`.

## Consequences

- Intent evaluation owns prompt-quality evidence.
- Enforcement continues to own the baked prompt and schema.
- Intent CLI `.experiments/` remains appropriate for command-integration evidence,
  not semantic-review quality fixtures.
- Tool tests may package or copy these fixtures, but must not become the source
  of truth for semantic-review intent.
