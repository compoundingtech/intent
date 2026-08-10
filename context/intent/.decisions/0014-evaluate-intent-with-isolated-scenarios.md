# Evaluate Intent with isolated scenarios

Status: accepted

## Context

After defining the root Intent contract and renaming `grill-intent`, the user asked to run an
end-to-end eval in a separate agent and to create a sub-Intent for the eval
approach.

## Evidence and Argument

The root Intent contract is self-recursive and skill-driven, so paper review is not enough.
An isolated scenario can exercise artifact routing, decision evidence, glossary
handling, open questions, references, experiments, deltas, and skill behavior
without polluting the real repo. Running it in a separate agent gives a partial
independent critique signal.

## Options

| Option | Tradeoffs |
| --- | --- |
| Review the docs manually only | Fast, but misses usability failures in the actual procedure. |
| Eval directly in the repo | Realistic, but risks polluting tracked files and confusing real Intent with test artifacts. |
| Eval in isolated temporary scenarios | Produces inspectable evidence while keeping durable findings review-gated. |

## Decision

Evaluate Intent and Intent skills through isolated temporary scenarios. The eval
worker must not modify tracked repo files; it reports evidence and proposed Intent
updates for the owner agent to review and apply.
