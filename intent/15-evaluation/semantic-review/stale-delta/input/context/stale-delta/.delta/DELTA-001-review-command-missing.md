# DELTA-001 Review command missing

Status: open

## Divergence

The VRS says `axe vrs review` should exist, but the implementation does not
provide it yet.

## Current Evidence

Completed in commit `abc1234`: `axe vrs review` now runs through the Coding
Agent Invocation Contract and emits `axe.vrs.review.v1`.

## Required Update

Keep this delta as a reminder to check review behavior later.
