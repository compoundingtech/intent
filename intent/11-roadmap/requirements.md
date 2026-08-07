# VRS Roadmap — Requirements

## Context

- This child node defines the contract for optional `roadmap.md` files.
- It refines [VRS-R15](../requirements.md).

## Requirements

### Must Preserve Future Direction Without Creating Contract

- **VRS.ROAD-R01 Non-normative:** Roadmap entries must not constrain current
  implementation.
- **VRS.ROAD-R02 Future direction:** A roadmap entry must describe a plausible
  later capability, phase, integration, or system direction.
- **VRS.ROAD-R03 Promotion required:** A roadmap entry becomes normative only
  when promoted into requirements, spec, or a decision record.

### Must Not Hide Current Work

- **VRS.ROAD-R04 Not drift:** Known current contract/reality divergence belongs
  in `.delta/`.
- **VRS.ROAD-R05 Not uncertainty:** Unresolved design questions belong in
  `open-questions.md`.
- **VRS.ROAD-R06 Not backlog:** Active implementation tasks belong in the
  project planning system, not VRS roadmap.
