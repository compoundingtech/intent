# VRS Experiments — Requirements

## Context

- This child node defines the contract for `.experiments/` companion
  directories. The directory name is historical shorthand; records may capture
  any concrete validation evidence, not only lab-style experiments.
- It refines [VRS-R09](../requirements.md) and [VRS-R10](../requirements.md).

## Requirements

### Must Capture Evidence

- **VRS.EXP-R01 Evidence trail:** Experiment records must record validation
  evidence, not plans or unresolved tasks.
- **VRS.EXP-R02 Reproducible method:** An experiment record must state enough method
  for a reader to understand how the conclusion was reached.
- **VRS.EXP-R03 Clear conclusion:** An experiment record must state what VRS
  artifact should change, if any.
- **VRS.EXP-R04 Assumption validation:** Experiments, benchmarks, prototypes,
  proofs, research, user confirmations, independent critique, or e2e checks
  should be used when they are the best feasible way to validate a load-bearing
  assumption.
- **VRS.EXP-R05 Focused records:** Experiment records should be split by major
  question, hypothesis, or validation method when aggregation makes the evidence
  hard to scan or promote.

### Must Stay Companion-Only

- **VRS.EXP-R06 Lazy directory:** `.experiments/` exists only when it contains
  real experiment records.
- **VRS.EXP-R07 Not normative:** Accepted findings must be promoted into
  requirements, spec, ontology, or decisions.
