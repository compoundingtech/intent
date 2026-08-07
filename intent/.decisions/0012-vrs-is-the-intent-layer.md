# VRS is the intent layer

Status: accepted

## Context

The Notion notes suggested describing VRS as the "intent layer" or intent
programming. The meta-VRS needed to decide whether that is a rename, a concept,
or out of scope.

## Evidence and Argument

The root vision already describes VRS as durable structured knowledge that
constrains skills, plans, and code. "Intent layer" captures that role well, but
renaming the artifact system would create churn and ambiguity because `VRS`
already names the concrete document contract.

## Options

| Option | Tradeoffs |
| --- | --- |
| Rename VRS to intent layer | Conceptually rich, but breaks existing skill and repo language. |
| Ignore intent-layer framing | Avoids new terminology, but loses a useful explanation of VRS's role. |
| Use intent layer as conceptual role | Clarifies purpose while preserving VRS as the artifact/system name. |

## Decision

Describe VRS as the project's **intent layer**: durable, structured intent that
constrains code, plans, skills, and agent behavior without replacing
implementation truth. Keep `VRS` as the concrete artifact and system name.
