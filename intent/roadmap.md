# VRS — Roadmap

Non-normative future direction. Entries do not constrain implementation until
promoted into requirements, spec, or a decision record.

## Planning Integration

- **Planning-tool integration:** VRS should eventually integrate with whichever
  tool owns durable plan intent, so long-running agent work can reference VRS
  artifacts, propose VRS changes, attach VRS evidence, and route durable
  learning back into the intent layer. Plan authority is external to this
  repository, so the integration is a consumer contract rather than a shared
  data model.
  - Trigger: an external planning authority is the normal durable work surface
    for agents, and exposes a stable reference for a plan and a step.
  - Promotion target: requirements, spec, or a dedicated integration child VRS.

## Structured Intent Layer

- **Structured VRS representation:** VRS should eventually move beyond a brittle
  collection of Markdown conventions toward a more structured intent layer with
  symbolic references, typed records, checked schemas, and tooling support for
  restructuring, renumbering, and cross-reference maintenance.
  - Current policy: roadmap only. Markdown artifacts remain normative until
    enforcement evidence shows that additional machine-readable structure is
    worth the authoring and migration cost.
  - Trigger: evidence shows a structured representation improves VRS
    correctness and maintainability without making authoring too heavy.
  - Promotion target: requirements, spec, enforcement, or a dedicated structured
    VRS child node.
- **Typed graph v0:** Before a full typed-source model, VRS should define a
  small Markdown-authored graph contract for nodes, IDs, references, refinement
  edges, statuses, and evidence edges. Markdown remains authoritative; generated
  graph JSON is derived.
  - Trigger: deterministic lint needs structure that cannot be inferred
    reliably from prose alone.
  - Promotion target: enforcement spec or a dedicated structured VRS child node.
