# VRS Open Questions — Requirements

## Context

- This child node defines the contract for `open-questions.md` files.
- It refines [VRS-R14](../requirements.md).

## Requirements

### Must Track Unresolved Uncertainty

- **VRS.OQ-R01 Genuine uncertainty:** An open question must describe unresolved
  design uncertainty, not a task, future idea, or confirmed implementation
  mismatch.
- **VRS.OQ-R02 Resolution path:** Each question must state what kind of evidence
  or decision would resolve it.
- **VRS.OQ-R03 Spec link:** When a question blocks a spec detail, the spec must
  reference it with a `DQ` identifier.

### Must Stay Fresh

- **VRS.OQ-R04 Prune resolved questions:** Resolved questions must be removed
  from `open-questions.md` in the same change that records the resolution.
- **VRS.OQ-R05 Move to owner:** A resolved answer must move to the owning VRS
  artifact when it affects durable system truth.
- **VRS.OQ-R06 Not a roadmap:** Far-future direction belongs in `roadmap.md`
  unless there is a concrete current design uncertainty to resolve.
- **VRS.OQ-R07 Drafting coverage:** During initial VRS drafting,
  `open-questions.md` may hold temporary coverage questions for areas not yet
  explored.
- **VRS.OQ-R08 Resolution pressure:** Agents must try to resolve every open
  question before finalizing VRS work unless the question has a clear blocker.
- **VRS.OQ-R09 Blocker explicit:** A blocked open question must name what
  external input, experiment, research, implementation fact, or decision would
  unblock it.
