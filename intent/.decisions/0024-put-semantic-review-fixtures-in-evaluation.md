# Put semantic review fixtures in evaluation

Status: accepted

## Context

After placing the baked semantic-review prompt and schema in
`context/vrs/16-enforcement/`, VRS needs a home for the fixtures that tune and
regression-test those assets.

The ownership question is whether semantic-review fixtures belong with the VRS
evaluation subsystem, enforcement implementation evidence, Axe command
experiments, or tool test fixtures.

## Evidence and Argument

The evaluation subsystem already owns isolated scenarios, evidence-backed
findings, and recommendation shape for VRS contract changes. Semantic-review
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
| `context/vrs/15-evaluation/semantic-review/` | Keeps prompt-quality evidence with the evaluation subsystem and preserves VRS ownership, but requires a clear distinction between tracked fixtures and temporary eval output. |
| `context/vrs/16-enforcement/.experiments/` | Keeps fixtures near prompt/schema, but mixes reusable eval cases with enforcement implementation evidence. |
| `context/coding-agents/14-axe/12-vrs/.experiments/` | Good for CLI and CAIC integration prototypes, but makes Axe look like the owner of semantic-review quality. |
| `flakes/axe/tests/fixtures/vrs-review/` | Easy for tool tests, but separates semantic eval intent from the VRS source of truth. |

## Decision

Semantic-review eval fixtures live under
`context/vrs/15-evaluation/semantic-review/`.

Tracked fixtures are canonical inputs and expected outcomes. Eval runs must copy
or materialize them into isolated temporary workspaces before invoking
`axe vrs review`.

## Consequences

- VRS evaluation owns prompt-quality evidence.
- Enforcement continues to own the baked prompt and schema.
- Axe VRS `.experiments/` remains appropriate for command-integration evidence,
  not semantic-review quality fixtures.
- Tool tests may package or copy these fixtures, but must not become the source
  of truth for semantic-review intent.
