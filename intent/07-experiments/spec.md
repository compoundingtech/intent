# Intent Experiments — Spec

This document specifies `.experiments/` directories. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Structure

```text
.experiments/
  <date-or-topic>-<slug>.md
```

Record format:

```markdown
# <Experiment title>

## Question

## Method

## Result

## Conclusion

## Intent Impact
```

Use `.experiments/` for validation evidence records: prototypes, benchmarks,
e2e validation, research reports, proofs, user confirmations, independent
critique, and failed approaches whose evidence should remain available after
the normative Intent artifacts are updated.

Keep records focused on one major question, hypothesis, or validation method.
Split a large evidence file when it becomes hard to answer "what was tested,
what happened, and what Intent artifact changed?" without reading unrelated
sections. Aggregate only when the evidence is inseparable or the comparison
itself is the experiment.

When an experiment validates an assumption, link the assumption ID in
`Intent Impact`. When an assumption cannot yet be validated, leave the gap in
`open-questions.md` with the blocker rather than recording a speculative
experiment.
