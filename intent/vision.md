# VRS — Vision

## The Need

VRS exists because durable system knowledge needs a home that is more precise
than prose notes and less procedural than an agent skill. The documentation
system must help humans and agents preserve intent, constraints, tradeoffs, and
implementation shape without confusing those artifacts with plans or temporary
working state.

## The Vision

- VRS is a self-describing documentation contract: the VRS system is itself
  specified with VRS.
- VRS is the project's intent layer: durable, structured intent that constrains
  code, plans, skills, and agent behavior without replacing implementation
  truth.
- VRS documents are small, composable, and hierarchical enough that each file
  has one job.
- VRS separates normative system truth from supporting evidence, references,
  open questions, and PR-local scaffolding.
- VRS gives coding agents a precise contract for where to record discoveries
  while keeping main-branch documentation clean and durable.

## What This Is Not

- VRS is not a project plan, milestone tracker, or backlog.
- VRS is not a replacement for implementation-owned source code.
- VRS is not a generic README convention.
- VRS is not limited to architecture; it also covers product, operational,
  data, interface, and validation decisions when they shape a system.

## Success Criteria

1. A reader can determine which VRS artifact owns a fact without relying on
   agent-specific skill text.
2. A VRS tree can scale from one small topic to a hierarchy of child nodes
   without duplicating upstream intent.
3. Proposed decisions, experiments, references, and deltas have clear lifecycles
   and do not pollute the durable contract.
4. Operational skills such as `vrs.md` and `grill-vrs` can become thin
   procedures over the normative `context/vrs/` contract.
