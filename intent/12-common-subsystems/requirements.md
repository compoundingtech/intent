# VRS Common Subsystems — Requirements

## Context

- This child node defines common VRS subsystem candidates.
- It refines [VRS-R04](../requirements.md) and [VRS-R17](../requirements.md).

## Requirements

### Must Be Subsystems, Not Companion Files

- **VRS.SUB-R01 Child node shape:** A common subsystem must be modeled as a
  child VRS node with its own `requirements.md` and `spec.md`.
- **VRS.SUB-R02 No generic data-model file:** Data models must not use a generic
  top-level `datamodel.md`; substantial data models become child VRS nodes.
- **VRS.SUB-R03 Lazy creation:** Common subsystem nodes are created only when
  the topic has enough scope to justify independent requirements and spec.

### Must Cover Recurring Design Surfaces

- **VRS.SUB-R04 Interface surfaces:** CLI commands, config files, APIs, UI
  routes, and other user/system surfaces should get a subsystem when their
  contract is non-trivial.
- **VRS.SUB-R05 Verification loops:** Test architecture, validation loops,
  conformance, benchmarks, and e2e checks should get a subsystem when they shape
  the system.
- **VRS.SUB-R06 Integrations:** External systems should get a subsystem when
  they impose meaningful assumptions, constraints, references, auth, or
  compatibility boundaries.
- **VRS.SUB-R07 Operations and bounds:** Runtime operation, observability,
  rollout, recovery, capacity, disk, memory, CPU, or latency bounds should get a
  subsystem when they need explicit design.
