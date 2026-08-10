# Intent Intuition — Requirements

## Context

- This child node defines the contract for `intuition.md` files in Intent nodes.
- It refines [INTENT-R01](../requirements.md) and [INTENT-R05](../requirements.md).

## Requirements

### Must Build the Mental Model

- **INTENT.INT-R01 Narrative entry point:** `intuition.md` is the Intent node's
  narrative entry point and replaces Intent use of `README.md`.
- **INTENT.INT-R02 Narrative model:** Intuition must explain the model a reader
  needs before reading formal docs.
- **INTENT.INT-R03 System map:** Intuition may include the node's map and reading
  order when that helps orientation.

### Must Stay Non-Normative

- **INTENT.INT-R04 No hidden constraints:** Normative constraints must live in
  `requirements.md` or `spec.md`, not only in intuition.
- **INTENT.INT-R05 No navigation filler:** Intuition must explain the system, not
  merely list files.
