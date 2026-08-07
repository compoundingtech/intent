# VRS Review Smells — Requirements

## Context

- This child node defines bad patterns agents must check when creating or
  editing VRS.
- It refines [VRS-R18](../requirements.md).

## Requirements

### Must Guide Review

- **VRS.SMELL-R01 Normative checks:** Review smells must be treated as VRS
  review rules, not optional style preferences.
- **VRS.SMELL-R02 Fix ownership:** When a smell appears, the fix must move the
  content to the correct VRS owner or delete it.
- **VRS.SMELL-R03 No smell artifact:** Individual VRS topics must not create a
  recurring `smells.md` artifact; smell checks live in the meta-VRS contract.

### Must Catch Common Drift

- **VRS.SMELL-R04 Lifecycle freshness:** Stale open questions, roadmap items,
  deltas, proposed decisions, experiments, and references must be updated,
  promoted, pruned, or deleted.
- **VRS.SMELL-R05 Artifact boundary:** Content in the wrong VRS artifact must be
  moved rather than duplicated.
