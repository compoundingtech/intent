# Intent Roadmap — Requirements

## Context

- This child node defines the contract for optional `roadmap.md` files.
- It refines [INTENT-R15](../requirements.md).

## Requirements

### Must Preserve Future Direction Without Creating Contract

- **INTENT.ROAD-R01 Non-normative:** Roadmap entries must not constrain current
  implementation.
- **INTENT.ROAD-R02 Future direction:** A roadmap entry must describe a plausible
  later capability, phase, integration, or system direction.
- **INTENT.ROAD-R03 Promotion required:** A roadmap entry becomes normative only
  when promoted into requirements, spec, or a decision record.

### Must Not Hide Current Work

- **INTENT.ROAD-R04 Not drift:** Known current contract/reality divergence belongs
  in `.delta/`.
- **INTENT.ROAD-R05 Not uncertainty:** Unresolved design questions belong in
  `open-questions.md`.
- **INTENT.ROAD-R06 Not backlog:** Active implementation tasks belong in the
  project planning system, not Intent roadmap.
