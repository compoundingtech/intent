# VRS Open Questions — Spec

This document specifies `open-questions.md` files. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Structure

```markdown
# <Topic> — Open Questions

Only genuinely unresolved design questions. Resolved questions move to their
owning VRS artifact or are deleted.

## DQ1: <Question>

- Blocks: <spec section or requirement, if any>
- Resolution signal: <evidence, decision, experiment, or user answer>
- Blocker: <external input, experiment, research, implementation fact, or decision; omit if not blocked>
- Lean: <current best answer, if useful>
```

Use `DQ` IDs when questions are referenced from `spec.md`. If a question is
answered, update the spec/requirements/ontology/decision/experiment/reference or
roadmap entry that now owns the content, then remove the question.

Do not keep a resolved-question archive. Git history preserves prior questions.

## Drafting Coverage

During initial VRS drafting, use `open-questions.md` to avoid losing coverage
areas while the conversation or investigation dives deep into one branch.
Coverage questions should be specific enough to act on:

```markdown
## DQ3: What are the resource bounds for this system?

- Blocks: requirements constraints
- Resolution signal: measure current disk/memory/CPU behavior or cite an
  operational constraint
```

Before finalizing VRS work, walk every open question. Resolve it into the owning
artifact when possible. Leave it open only if it has a clear blocker, and record
that blocker in the question.

When the open question exists because a load-bearing assumption is not yet
validated, name the assumption and the best feasible validation path.
