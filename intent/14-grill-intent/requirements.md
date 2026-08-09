# Intent Grill Intent — Requirements

## Context

- This child node defines how the Intent contract composes with the `grill-intent`
  interview skill.
- It refines [INTENT-R12](../requirements.md) and [INTENT-R23](../requirements.md).

## Requirements

### Must Apply Intent, Not Redefine It

- **INTENT.GRILL-R01 Intent contract source:** `intent/` is the source of truth
  for artifact structure, lifecycle, and review rules.
- **INTENT.GRILL-R02 Procedural skill:** `grill-intent` owns the interview procedure:
  questioning, pressure-testing, validation, and inline Intent updates.
- **INTENT.GRILL-R03 Thin skill:** The operational skill must avoid duplicating
  detailed artifact rules that already live in the root Intent contract.

### Must Preserve Interview Discipline

- **INTENT.GRILL-R04 One question:** The procedure asks one question at a time and
  waits for feedback.
- **INTENT.GRILL-R05 Recommended answer:** Each question includes the agent's
  recommended answer and tradeoff framing.
- **INTENT.GRILL-R06 Explore before asking:** If code, Intent, references, or tooling
  can answer a question, the agent investigates before asking the user.
- **INTENT.GRILL-R07 Update as understanding crystallizes:** Resolved terms,
  requirements, constraints, decisions, experiments, references, deltas, and
  open questions are captured in the correct Intent artifact as they become clear.
- **INTENT.GRILL-R08 Ontology handling:** The procedure resolves fuzzy,
  conflicting, or overloaded terms and updates `ontology.md` using the
  root Intent ontology contract.

### Must Use the Preferred Name

- **INTENT.GRILL-R09 Skill name:** The skill name is `grill-intent`.
