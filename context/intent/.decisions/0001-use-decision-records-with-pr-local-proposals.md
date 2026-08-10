# Use decision records with PR-local proposals

Status: accepted

## Context

Intent needs a durable log for consequential choices, but those choices are not
limited to architecture. Agents also need a branch-local way to keep working
through unresolved decisions without letting tentative records merge to main.

## Evidence and Argument

The repo already uses `decisions/` and `.decisions/` widely for non-architecture
choices. The user's Notion notes asked about `.adr`, proposed records for
autonomous mode, and whether ADR is general enough. ADR practice is recognizable
but usually architecture-framed; Intent explicitly covers product, operational,
data, interface, validation, and design choices too.

## Options

| Option | Tradeoffs |
| --- | --- |
| `.adr/` | Familiar acronym, but misleading for non-architecture decisions. |
| `.decisions/accepted/` plus `.decisions/proposed/` | Lifecycle is explicit, but invites proposed records to merge and adds directory ceremony. |
| `.decisions/000N-*.md` plus PR-local `.decisions/.proposed/` | Broad, matches repo language, supports agent autonomy, and keeps main clean. |

## Decision

Use **Decision Record** as the canonical term and store durable records under
`.decisions/000N-<slug>.md`. Proposed records may live under
`.decisions/.proposed/` while a PR is open, but must be accepted, folded into
another Intent artifact, deferred to `open-questions.md`, or deleted before merge.
