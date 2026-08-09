# Intent Open Questions — Requirements

## Context

- This child node defines the contract for `open-questions.md` files.
- It refines [INTENT-R14](../requirements.md).

## Requirements

### Must Track Unresolved Uncertainty

- **INTENT.OQ-R01 Genuine uncertainty:** An open question must describe unresolved
  design uncertainty, not a task, future idea, or confirmed implementation
  mismatch.
- **INTENT.OQ-R02 Resolution path:** Each question must state what kind of evidence
  or decision would resolve it.
- **INTENT.OQ-R03 Spec link:** When a question blocks a spec detail, the spec must
  reference it with a `DQ` identifier.

### Must Stay Fresh

- **INTENT.OQ-R04 Prune resolved questions:** Resolved questions must be removed
  from `open-questions.md` in the same change that records the resolution.
- **INTENT.OQ-R05 Move to owner:** A resolved answer must move to the owning Intent
  artifact when it affects durable system truth.
- **INTENT.OQ-R06 Not a roadmap:** Far-future direction belongs in `roadmap.md`
  unless there is a concrete current design uncertainty to resolve.
- **INTENT.OQ-R07 Drafting coverage:** During initial Intent drafting,
  `open-questions.md` may hold temporary coverage questions for areas not yet
  explored.
- **INTENT.OQ-R08 Resolution pressure:** Agents must try to resolve every open
  question before finalizing Intent work unless the question has a clear blocker.
- **INTENT.OQ-R09 Blocker explicit:** A blocked open question must name what
  external input, experiment, research, implementation fact, or decision would
  unblock it.
