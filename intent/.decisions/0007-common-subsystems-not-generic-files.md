# Common subsystems, not generic files

Status: accepted

## Context

The Notion notes proposed a separate `datamodel.md` and listed other good Intent
areas: surface areas, data models, verification loops, external-system
assumptions, and resource bounds. The user clarified that data model should be a
common Intent subsystem rather than a generic companion file.

## Evidence and Argument

Data models and interfaces often need their own requirements, constraints,
specs, references, and verification. A single `datamodel.md` companion file
would create a special case and compete with the hierarchical Intent model. Common
subsystem candidates preserve the pattern without forcing every topic to create
them.

## Options

| Option | Tradeoffs |
| --- | --- |
| Add generic `datamodel.md` | Convenient for one concern, but special-cases data and does not generalize to interfaces or operations. |
| Keep all data model content in root `spec.md` | Simple for small systems, but large schemas overwhelm the root spec. |
| Treat data model and peers as common child subsystems | Scales through normal Intent composition and keeps companion files for lifecycle artifacts. |

## Decision

Treat recurring areas such as data model, interfaces, verification,
integrations, and operations as common subsystem candidates. A substantial data
model becomes a child Intent node with its own `requirements.md` and `spec.md`, not
`datamodel.md`.
