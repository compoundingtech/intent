# VRS Requirements — Requirements

## Context

- This child node defines the contract for `requirements.md` files in VRS nodes.
- It refines [VRS-R01](../requirements.md), [VRS-R03](../requirements.md), and
  [VRS-R06](../requirements.md).

## Requirements

### Must Be Testable

- **VRS.REQ-R01 Testable constraints:** Every requirement must describe an
  externally checkable constraint, not an implementation choice.
- **VRS.REQ-R02 Assumptions explicit:** Load-bearing assumptions must be listed
  with IDs that are unique in their declared scope.
- **VRS.REQ-R03 Assumptions validated:** Load-bearing assumptions must be
  validated in the best feasible way before they become durable VRS truth, or
  tracked as blocked open questions with a resolution signal.
- **VRS.REQ-R04 Tradeoffs explicit:** Accepted compromises must be listed with
  IDs that are unique in their declared scope unless they warrant a decision
  record.
- **VRS.REQ-R05 Constraints explicit:** Non-negotiable environmental,
  operational, platform, regulatory, resource, or integration limits must be
  listed as constraints rather than mixed into desired behavior.
- **VRS.REQ-R06 Referenced constraints:** Constraints derived from external
  systems, standards, APIs, vendors, or platform behavior must cite `.reference/`
  material.

### Must Stay Navigable

- **VRS.REQ-R07 Size bound:** A requirements file should stay below 30
  requirements and must stay below 40; larger scopes require child VRS nodes.
- **VRS.REQ-R08 Scoped IDs:** Requirements, assumptions, constraints, and
  tradeoffs must use IDs whose scope is clear from the artifact or namespace.
- **VRS.REQ-R09 Atomic consistency:** When IDs are renamed, renumbered, moved, or
  re-scoped, all cross-references must be updated in the same commit.
