# Use a narrow VRS enforcement subsystem

Status: accepted

## Context

After evaluating VRS and researching the remaining checklist items, the user was
unsure whether a broad tooling subsystem made sense. Focused probes found that
IDs, xrefs, compact records, delta upkeep, and Axe Plan integration have
different semantic owners.

## Evidence and Argument

Identifier and xref rules mostly belong to requirements/spec/evaluation until a
real resolver protocol exists. Compact decisions and experiments belong to their
artifact contracts and review smells. Delta upkeep belongs to the delta contract
plus `grill-vrs` procedure, with tooling only for mechanical checks. Axe Plan
integration belongs to the Axe Plans VRS, not to meta-VRS tooling.

The remaining cross-cutting concern with enough independent surface area is
enforcement: deterministic lint, semantic review, diagnostics, gates, and local
workflow integration for real repository VRS trees.

## Options

| Option | Tradeoffs |
| --- | --- |
| No new child node | Minimal, but enforcement rules and diagnostics scatter across artifact contracts. |
| Broad `16-tooling/` node | Convenient bucket, but mixes unrelated semantics, integrations, and implementation plans. |
| Narrow `16-enforcement/` node | Gives repo hygiene checks a home while preserving semantic ownership in existing nodes. |

## Decision

Create `16-enforcement/` as a narrow meta-VRS child node. It owns VRS hygiene
checks, diagnostics, and gate semantics. It does not own artifact semantics,
`axe plan` integration, isolated evaluation, or every VRS-related tool.
