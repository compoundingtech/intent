# VRS Delta — Requirements

## Context

- This child node defines the contract for `.delta/` companion directories.
- It refines [VRS-R11](../requirements.md) and [VRS-R13](../requirements.md).

## Requirements

### Must Track Drift Explicitly

- **VRS.DELTA-R01 Confirmed divergence:** A delta must describe a confirmed,
  currently open divergence between normative VRS and implementation, observed
  behavior, or verification evidence.
- **VRS.DELTA-R02 Direction clear:** Each delta must say whether the spec or
  implementation is expected to change.
- **VRS.DELTA-R03 Stable IDs:** Deltas must have stable identifiers so agents
  can update or close them precisely.

### Must Stay Fresh

- **VRS.DELTA-R04 Drift only:** Delta records are for confirmed contract/reality
  divergence, not general tasks or future work.
- **VRS.DELTA-R05 Closure required:** A delta closes by updating the
  implementation, updating VRS, or recording an accepted decision that changes
  the contract.
- **VRS.DELTA-R06 Prune resolved records:** Completed, stale, duplicate, or
  no-longer-true deltas must be removed instead of retained as history.
- **VRS.DELTA-R07 Review on related edits:** Any change to a referenced VRS
  artifact or implementation surface must update or close affected deltas in the
  same change.
- **VRS.DELTA-R08 Main may contain open deltas:** Open delta files may exist on
  main when they represent confirmed current drift; closed, stale, speculative,
  or completed delta files must not.
