# VRS Decisions — Requirements

## Context

- This child node defines the contract for VRS decision records.
- It refines [VRS-R07](../requirements.md) and [VRS-R08](../requirements.md).

## Requirements

### Must Record Consequential Choices

- **VRS.DEC-R01 Broad scope:** Decision records may cover architecture,
  product, operational, data, interface, validation, or design choices.
- **VRS.DEC-R02 Admission rule:** A durable decision record must be costly or
  confusing to reverse, surprising without context, and the result of a real
  tradeoff.
- **VRS.DEC-R03 No changelog:** Routine implementation changes must not become
  decision records.
- **VRS.DEC-R04 Evidence-backed:** A durable decision must include the best
  available evidence, proof, argument, research, experiment, implementation
  fact, or user input backing the chosen approach.
- **VRS.DEC-R05 Principled options:** A durable decision must list the best
  principled options considered and their tradeoffs.
- **VRS.DEC-R06 Choice rationale:** A durable decision must explain why the
  chosen option is best under the VRS context and constraints.
- **VRS.DEC-R07 Multiple evidence forms:** A durable decision must include at
  least one applicable evidence form and should include multiple independent
  evidence forms when the decision is important, risky, or expensive to reverse.
- **VRS.DEC-R08 Compact rationale:** A decision record must summarize the
  decisive evidence and rationale rather than embedding bulky experiment logs,
  benchmark tables, transcripts, or implementation backlogs.

### Must Preserve Main-Branch Cleanliness

- **VRS.DEC-R09 Durable main:** Main branch may contain accepted, deprecated,
  and superseded decision records.
- **VRS.DEC-R10 PR-local proposals:** Proposed records may live under
  `.decisions/.proposed/` during a PR only.
- **VRS.DEC-R11 Proposal exit:** Every proposed record must be accepted,
  folded into another artifact, moved to `open-questions.md`, or deleted before
  merge.
