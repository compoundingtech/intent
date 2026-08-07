# Validate load-bearing assumptions

Status: accepted

## Context

The Notion notes asked for e2e verification, research, prototypes, and gap
finding during VRS design. The user clarified that assumptions should always be
validated in the best possible way, including proofs and independent critique.

## Evidence and Argument

Assumptions are where weak VRS documents often smuggle uncertainty into durable
truth. Validation can take many shapes depending on the claim: user confirmation
for intent, references for external behavior, benchmarks or e2e experiments for
runtime claims, prototypes for feasibility, proofs for deterministic claims,
implementation evidence for current behavior, and subagent critique for
independent pressure-testing.

## Options

| Option | Tradeoffs |
| --- | --- |
| Require experiments for every assumption | Strong evidence, but too heavy and wrong for assumptions better validated by proof, reference, or user confirmation. |
| Only record assumptions without validation | Low friction, but lets speculation harden into VRS truth. |
| Require best feasible validation or a blocked open question | Proportional rigor while preserving progress when validation is not yet possible. |

## Decision

Load-bearing assumptions must be validated in the best feasible way before they
become durable VRS truth. Valid forms include user confirmation, research,
benchmarks, e2e experiments, prototypes, proofs, implementation evidence, or
independent critique from another agent. If validation is not feasible yet, the
assumption remains an open question with a clear blocker and resolution signal.
