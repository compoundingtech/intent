# VRS Evaluation — Requirements

## Context

- This child node defines how to evaluate the VRS system and VRS skills.
- It refines [VRS-R25](../requirements.md).

## Requirements

### Must Evaluate End To End

- **VRS.EVAL-R01 Isolated scenario:** An eval must run in an isolated temporary
  project or scenario outside tracked project files.
- **VRS.EVAL-R02 Realistic surface area:** The scenario must exercise multiple
  VRS artifact kinds, including requirements, spec, ontology, decisions, and at
  least one companion artifact.
- **VRS.EVAL-R03 Skill application:** The eval must apply the relevant skill
  procedure, especially `grill-vrs` for unclear or consequential design.

### Must Produce Evidence

- **VRS.EVAL-R04 Artifact evidence:** The eval must preserve or report the temp
  artifact paths so findings can be inspected.
- **VRS.EVAL-R05 Gap classification:** Findings must classify gaps by owning
  VRS artifact: requirement, spec, ontology, decision, open question, delta,
  experiment, reference, roadmap, review smell, enforcement, or skill update.
- **VRS.EVAL-R06 No tracked edits by eval worker:** The eval worker must not
  modify tracked repo files; durable findings are applied by the owner agent
  after review.

### Must Support Iteration

- **VRS.EVAL-R07 Recommendation shape:** Each recommendation must include the
  evidence, options or alternatives when relevant, tradeoffs, and suggested VRS
  destination.
- **VRS.EVAL-R08 Milestone evidence:** Significant VRS skill or contract changes
  should be followed by at least one isolated scenario eval before being treated
  as stable.
