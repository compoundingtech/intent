# Glossary handling lives in Grill Intent

Status: superseded by [0032](./0032-rename-glossary-artifact-to-ontology.md)

## Context

The Notion notes asked to embrace glossary handling. The user agreed that this
should be part of `grill-intent` rather than a separate glossary skill by default.

## Evidence and Argument

Glossary updates happen naturally during the interview loop: terms become fuzzy,
conflicting, or overloaded while requirements and decisions are being shaped.
Splitting that into a separate skill would add routing overhead before glossary
work has shown enough independent complexity to justify it.

## Options

| Option | Tradeoffs |
| --- | --- |
| Separate glossary skill now | Clear ownership, but premature and interrupts the design interview flow. |
| Keep glossary rules only in root Intent contract | Normative, but does not say who applies them during interviews. |
| Make `grill-intent` apply the glossary contract | Simple flow and one less skill, while preserving the option to split later. |

## Decision

`grill-intent` handles glossary work by applying `intent/04-glossary/`.
A separate glossary skill is created only if glossary work becomes complex
enough to need an independently reusable procedure.
