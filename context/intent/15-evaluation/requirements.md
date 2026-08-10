# Intent Evaluation — Requirements

## Context

- This child node defines how to evaluate the Intent system and Intent skills.
- It refines [INTENT-R25](../requirements.md).

## Requirements

### Must Evaluate End To End

- **INTENT.EVAL-R01 Isolated scenario:** An eval must run in an isolated temporary
  project or scenario outside tracked project files.
- **INTENT.EVAL-R02 Realistic surface area:** The scenario must exercise multiple
  Intent artifact kinds, including requirements, spec, ontology, decisions, and at
  least one companion artifact.
- **INTENT.EVAL-R03 Skill application:** The eval must apply the relevant skill
  procedure, especially `grill-intent` for unclear or consequential design.

### Must Produce Evidence

- **INTENT.EVAL-R04 Artifact evidence:** The eval must preserve or report the temp
  artifact paths so findings can be inspected.
- **INTENT.EVAL-R05 Gap classification:** Findings must classify gaps by owning
  Intent artifact: requirement, spec, ontology, decision, open question, delta,
  experiment, reference, roadmap, review smell, enforcement, or skill update.
- **INTENT.EVAL-R06 No tracked edits by eval worker:** The eval worker must not
  modify tracked repo files; durable findings are applied by the owner agent
  after review.

### Must Support Iteration

- **INTENT.EVAL-R07 Recommendation shape:** Each recommendation must include the
  evidence, options or alternatives when relevant, tradeoffs, and suggested Intent
  destination.
- **INTENT.EVAL-R08 Milestone evidence:** Significant Intent skill or contract changes
  should be followed by at least one isolated scenario eval before being treated
  as stable.
