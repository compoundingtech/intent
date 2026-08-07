# Add constraints section to requirements

Status: accepted

## Context

The Notion notes asked to add a constraints section to `requirements.md` and
called out external systems, resource bounds, and references as important VRS
areas. Requirements needed a way to distinguish desired behavior from
non-negotiable boundaries.

## Evidence and Argument

Platform limits, API limits, compliance rules, and disk/memory/CPU bounds are
not "what the system does"; they are constraints the system must respect.
Treating them as normal requirements blurs ownership and makes source evidence
harder to trace. The user also noted constraints are directly related to
references.

## Options

| Option | Tradeoffs |
| --- | --- |
| Keep constraints under `## Requirements` | Fewer sections, but mixes desired behavior with external limits. |
| Add `## Constraints` | Clearer semantics and better reference linkage, with one more section. |
| Put constraints only in `.reference/` | Preserves evidence, but hides normative limits in non-normative material. |

## Decision

VRS requirements files include a first-class `## Constraints` section for
non-negotiable environmental, operational, platform, regulatory, resource, or
integration limits. Constraints derived from external systems must cite
`.reference/` material. The constraint is normative; the reference preserves the
external evidence.
