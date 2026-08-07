# VRS Requirements — Spec

This document specifies `requirements.md` files. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Structure

```markdown
# <Topic> — Requirements

## Context

## Assumptions

- **<NS>-A01 Short name:** ...
  - Validation: <user confirmation | reference | experiment | benchmark | prototype | proof | subagent critique | implementation evidence>

## Constraints

- **<NS>-C01 Short name:** ...

## Acceptable Tradeoffs

- **<NS>-T01 Short name:** ...

## Requirements

### Must <Constraint Group>

- **<NS>-R01 Short name:** ...
```

Use a semantic namespace (`VRS`, `VRS.REQ`, `CVG.ACT`) when references cross
artifact, child-node, or project boundaries. Local IDs such as `R01` are
acceptable inside one declared requirements scope when every reference is
unambiguous from context. Numeric directory prefixes do not appear in IDs.

IDs are not permanent public API. They may be renamed, renumbered, or re-scoped
when the VRS shape improves, but the repository commit must remain atomically
consistent: every reference in that commit resolves to the intended current
artifact and clause.

Requirements say what the system must do. Constraints say what the system must
respect: platform limits, resource bounds, regulatory/compliance rules,
third-party API limits, compatibility requirements, or operational envelopes.

When a constraint comes from an external system or source, link the relevant
`.reference/` record from the constraint. The requirement file states the
constraint; `.reference/` preserves the external evidence behind it.

Assumptions need validation proportional to how load-bearing they are. Valid
forms include user confirmation, `.reference/` research, `.experiments/`
benchmarks or e2e validation, prototypes, proofs, implementation evidence, or
independent self-critique from another agent. If the best feasible validation
cannot be done yet, create an `open-questions.md` entry with the blocker and
resolution signal instead of treating the assumption as settled.

If a bullet says how to build it, move it to `spec.md`. If a tradeoff needs more
than a one-line explanation, promote it to a decision record.
