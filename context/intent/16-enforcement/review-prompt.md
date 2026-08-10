# Intent Semantic Review Prompt

You are running `intent review`.

Review only the supplied Intent artifacts, deterministic diagnostics, and
normative Intent contracts. Return JSON that matches `axe.intent.review.v1`. Do not
edit files, do not propose patches, and do not rely on repository files that
were not supplied as context.

## Review Scope

Classify semantic Intent issues that deterministic lint cannot prove safely:

- `vision.md` contains implementation, architecture, tools, migration plans, or
  other mechanisms instead of durable intent.
- `requirements.md` contains implementation mechanism instead of testable
  constraints.
- `spec.md` is not detailed enough to implement, fails to trace to
  requirements, or contains rationale that belongs in a decision record.
- `ontology.md` contains behavior, API shape, or decision history.
- `.decisions/` records lack substantive context, evidence, options,
  tradeoffs, or rationale.
- `.decisions/.proposed/` contains records that appear ready to merge without
  being accepted, folded, deferred, or deleted.
- `.experiments/` contains plans instead of evidence, or aggregates unrelated
  questions into one unclear conclusion.
- `.reference/` contains copied source material without explaining Intent impact.
- `.delta/` contains stale, completed, speculative, duplicate, or vague entries.
- `open-questions.md` contains resolved questions or questions without a real
  blocker.
- `roadmap.md` contains current contract rather than non-normative future
  direction.
- Empty companion directories or README-style Intent entry points appear where the
  root Intent contract does not allow them.

Use the review-smell catalog in `context/intent/13-review-smells/spec.md` as the
primary rubric when it is supplied.

## Finding Rules

Only report findings that are actionable and grounded in supplied evidence.

Each finding must:

- name the owner artifact or Intent child node that should receive the fix;
- cite the concrete evidence that led to the finding;
- describe the smallest principled fix;
- use severity `error` only when the issue blocks a correct Intent update or would
  make automated enforcement unsafe;
- use severity `warning` for likely semantic drift that needs review;
- use severity `info` for non-blocking observations that help route future
  cleanup.

Do not repeat deterministic diagnostics unless the deterministic finding
reveals a semantic issue that needs separate review.

If the supplied artifacts are insufficient to decide, return no finding for that
question. Do not invent missing context.
