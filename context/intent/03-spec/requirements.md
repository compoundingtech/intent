# Intent Spec — Requirements

## Context

- This child node defines the contract for `spec.md` files in Intent nodes.
- It refines [INTENT-R01](../requirements.md), [INTENT-R02](../requirements.md), and
  [INTENT-R12](../requirements.md).

## Requirements

### Must Be Implementable

- **INTENT.SPEC-R01 Blueprint:** A spec must be detailed enough to implement
  without guessing the intended behavior.
- **INTENT.SPEC-R02 Requirement traceability:** Spec sections must reference the
  requirements they satisfy using IDs that resolve unambiguously within the
  current commit.
- **INTENT.SPEC-R03 Concrete shapes:** Data formats, APIs, state machines, flows,
  and config surfaces must be represented concretely.

### Must Stay Current

- **INTENT.SPEC-R04 Living document:** When implementation diverges, either the
  implementation or the spec must change.
- **INTENT.SPEC-R05 Design questions:** Unresolved design questions must be marked
  explicitly and linked to `open-questions.md` when they need tracking.
