# VRS — Intuition

*For: agents and maintainers shaping durable system docs · Assumes: Markdown
docs live beside code or under `context/` · Covers: the mental model for VRS as
a self-describing documentation system*

VRS is a compact contract for keeping design knowledge current without turning
docs into plans, changelogs, or essays. A VRS tree separates durable intent from
implementation detail:

```text
vision.md         why this exists and what success means
requirements.md   testable constraints
spec.md           how the system satisfies the constraints
ontology.md       canonical language
intuition.md      narrative model and reading frame
.decisions/       durable consequential decisions
.experiments/     evidence and validation trails
.reference/       external source material snapshots
.delta/           open contract/reality divergences
open-questions.md unresolved design uncertainty
roadmap.md        non-committed future direction
```

Not every VRS node has every file. Files and companion directories are lazy:
create them when there is real content, not to satisfy a template.

The narrative entry point for a VRS node is `intuition.md`, not `README.md`.
It builds the reader's mental model and may include the system map; it is part
of the VRS language rather than a generic repository convention. In directory
shape, it follows the formal `vision.md`/`requirements.md`/`spec.md` chain and
`ontology.md`.

VRS is hierarchical. A root VRS defines the system-level contract; child VRS
nodes refine one file kind, subsystem, mechanism, or concept when the parent
would otherwise become too large or ambiguous. The meta-VRS uses this recursively:
Vision, Requirements, Spec, Ontology, Decision Records, Experiments, Reference,
Delta, Open Questions, and Roadmap each deserve their own child VRS when their
contract needs more detail than the root can carry cleanly.

Decision records are durable. Proposed decisions are allowed during a PR so an
agent can continue working without blocking on every unresolved design choice,
but proposed records are PR-local scaffolding. Before merge, each proposed
decision is accepted into `.decisions/`, folded into the relevant VRS document,
or deleted.
