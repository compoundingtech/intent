# VRS Intuition — Requirements

## Context

- This child node defines the contract for `intuition.md` files in VRS nodes.
- It refines [VRS-R01](../requirements.md) and [VRS-R05](../requirements.md).

## Requirements

### Must Build the Mental Model

- **VRS.INT-R01 Narrative entry point:** `intuition.md` is the VRS node's
  narrative entry point and replaces VRS use of `README.md`.
- **VRS.INT-R02 Narrative model:** Intuition must explain the model a reader
  needs before reading formal docs.
- **VRS.INT-R03 System map:** Intuition may include the node's map and reading
  order when that helps orientation.

### Must Stay Non-Normative

- **VRS.INT-R04 No hidden constraints:** Normative constraints must live in
  `requirements.md` or `spec.md`, not only in intuition.
- **VRS.INT-R05 No navigation filler:** Intuition must explain the system, not
  merely list files.
