# Intent Decisions — Spec

This document specifies Intent decision records. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Durable Records

Durable records live in `.decisions/`:

```text
.decisions/
  0001-<slug>.md
  0002-<slug>.md
```

Minimum format:

```markdown
# <Decision title>

Status: accepted

## Context

What forced a choice.

## Evidence and Argument

Best available evidence, proof, research, experiment, implementation fact, user
input, or reasoning. Include at least one applicable form; use multiple
independent forms when the decision is important or risky.

## Options

| Option | Tradeoffs |
| --- | --- |
| A | ... |
| B | ... |

## Decision

Chosen option and why it is best under the current Intent context and constraints.
```

Statuses are `accepted`, `deprecated`, or `superseded by <id>`. Add
`Consequences` or follow-up constraints when downstream effects need to be
called out.

A short decision can keep each section to one paragraph or a small table, but it
must still show evidence, principled options, tradeoffs, and why the selected
option is best. At least one evidence form is required; more than one is
preferred when feasible. If that work is not done yet, keep the record proposed
or move the uncertainty to `open-questions.md`.

## Compactness

Decision records are the conclusion layer, not the evidence dump. Keep the
record focused on the choice a future reader needs to understand:

- summarize decisive evidence in prose or a small table;
- link bulky experiments, benchmarks, transcripts, research notes, proofs, or
  implementation evidence instead of embedding them;
- keep options to the principled alternatives that were genuinely considered;
- move follow-up work to the planning system, `.delta/`, `open-questions.md`, or
  the relevant requirements/spec artifact.

Longer records are acceptable for unusually risky or expensive choices, but
length must come from decision-relevant reasoning rather than raw supporting
material.

## Proposed Records

Proposed records live under `.decisions/.proposed/` only while a PR is open:

```text
.decisions/
  .proposed/
    <slug>.md
```

The proposed filename does not reserve a durable number. Promotion assigns the
next durable number after scanning existing `.decisions/000N-*.md` files.
