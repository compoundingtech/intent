# VRS Grill VRS — Requirements

## Context

- This child node defines how the VRS contract composes with the `grill-vrs`
  interview skill.
- It refines [VRS-R12](../requirements.md) and [VRS-R23](../requirements.md).

## Requirements

### Must Apply VRS, Not Redefine It

- **VRS.GRILL-R01 VRS contract source:** `context/vrs/` is the source of truth
  for artifact structure, lifecycle, and review rules.
- **VRS.GRILL-R02 Procedural skill:** `grill-vrs` owns the interview procedure:
  questioning, pressure-testing, validation, and inline VRS updates.
- **VRS.GRILL-R03 Thin skill:** The operational skill must avoid duplicating
  detailed artifact rules that already live in the meta-VRS.

### Must Preserve Interview Discipline

- **VRS.GRILL-R04 One question:** The procedure asks one question at a time and
  waits for feedback.
- **VRS.GRILL-R05 Recommended answer:** Each question includes the agent's
  recommended answer and tradeoff framing.
- **VRS.GRILL-R06 Explore before asking:** If code, VRS, references, or tooling
  can answer a question, the agent investigates before asking the user.
- **VRS.GRILL-R07 Update as understanding crystallizes:** Resolved terms,
  requirements, constraints, decisions, experiments, references, deltas, and
  open questions are captured in the correct VRS artifact as they become clear.
- **VRS.GRILL-R08 Ontology handling:** The procedure resolves fuzzy,
  conflicting, or overloaded terms and updates `ontology.md` using the
  meta-VRS ontology contract.

### Must Use the Preferred Name

- **VRS.GRILL-R09 Skill name:** The skill name is `grill-vrs`.
