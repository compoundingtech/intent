# VRS Grill VRS — Spec

This document specifies the `grill-vrs` procedure. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Relationship

```text
context/vrs/        normative intent-layer contract
grill-vrs           procedure that interrogates and updates the contract
```

VRS defines which artifact owns each fact. `grill-vrs` defines how an agent
arrives at those facts with a human or through code/research investigation.

## Procedure

1. Read the relevant VRS node and the meta-VRS contract.
2. Investigate code, references, experiments, and existing decisions before
   asking questions that local context can answer.
3. Ask one question at a time.
4. Include the recommended answer, options, tradeoffs, and evidence gaps.
5. Validate load-bearing assumptions with the best feasible evidence.
6. Resolve fuzzy, conflicting, or overloaded terms into `ontology.md`.
7. Update the owning VRS artifact as soon as understanding crystallizes.
8. Walk remaining open questions before finalizing; leave only blocked
   questions with explicit blockers.
9. Run the review-smell catalog before closing the session.

## Skill Reduction

When the operational skill is updated, it should keep procedural rules and link
to `context/vrs/` for artifact details. The skill should not carry independent
templates for decisions, ontology, requirements, experiments, deltas, or roadmap
when the meta-VRS already defines them.
