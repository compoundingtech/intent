# Intent Review Smells — Requirements

## Context

- This child node defines bad patterns agents must check when creating or
  editing Intent.
- It refines [INTENT-R18](../requirements.md).

## Requirements

### Must Guide Review

- **INTENT.SMELL-R01 Normative checks:** Review smells must be treated as Intent
  review rules, not optional style preferences.
- **INTENT.SMELL-R02 Fix ownership:** When a smell appears, the fix must move the
  content to the correct Intent owner or delete it.
- **INTENT.SMELL-R03 No smell artifact:** Individual Intent topics must not create a
  recurring `smells.md` artifact; smell checks live in the root Intent contract.

### Must Catch Common Drift

- **INTENT.SMELL-R04 Lifecycle freshness:** Stale open questions, roadmap items,
  deltas, proposed decisions, experiments, and references must be updated,
  promoted, pruned, or deleted.
- **INTENT.SMELL-R05 Artifact boundary:** Content in the wrong Intent artifact must be
  moved rather than duplicated.
