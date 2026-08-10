# Intent — Vision

## The Need

Intent exists because durable system knowledge needs a home that is more precise
than prose notes and less procedural than an agent skill. The documentation
system must help humans and agents preserve intent, constraints, tradeoffs, and
implementation shape without confusing those artifacts with plans or temporary
working state.

## The Vision

- Intent is a self-describing documentation contract: the Intent system is itself
  specified with Intent.
- Intent is the project's intent layer: durable, structured intent that constrains
  code, plans, skills, and agent behavior without replacing implementation
  truth.
- Intent documents are small, composable, and hierarchical enough that each file
  has one job.
- Intent separates normative system truth from supporting evidence, references,
  open questions, and PR-local scaffolding.
- Intent gives coding agents a precise contract for where to record discoveries
  while keeping main-branch documentation clean and durable.

## What This Is Not

- Intent is not a project plan, milestone tracker, or backlog.
- Intent is not a replacement for implementation-owned source code.
- Intent is not a generic README convention.
- Intent is not limited to architecture; it also covers product, operational,
  data, interface, and validation decisions when they shape a system.

## Success Criteria

1. A reader can determine which Intent artifact owns a fact without relying on
   agent-specific skill text.
2. An Intent tree can scale from one small topic to a hierarchy of child nodes
   without duplicating upstream intent.
3. Proposed decisions, experiments, references, and deltas have clear lifecycles
   and do not pollute the durable contract.
4. Operational skills such as `intent.md` and `grill-intent` can become thin
   procedures over the normative `context/intent/` contract.
