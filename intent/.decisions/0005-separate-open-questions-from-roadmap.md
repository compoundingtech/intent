# Separate open questions from roadmap

Status: accepted

## Context

The user confirmed that `open-questions.md` may exist on main, but resolved
questions should leave it. They also raised the need for a `roadmap.md` surface
for larger future ideas that should not become current spec.

## Evidence and Argument

Existing repo examples already use `open-questions.md` as an unresolved-design
surface and prune resolved questions. Future direction is a different concept:
it is not an unresolved blocker and not current contract. Mixing it into open
questions turns uncertainty tracking into a backlog.

## Options

| Option | Tradeoffs |
| --- | --- |
| Keep future ideas in `open-questions.md` | Fewer files, but conflates unresolved uncertainty with non-committed direction. |
| Put future ideas in `spec.md` | Keeps them close to design, but makes future direction look normative. |
| Add optional `roadmap.md` | Separates future direction from current contract and from unresolved questions. |

## Decision

Use `open-questions.md` for unresolved design uncertainty and optional
`roadmap.md` for non-normative future direction. Resolved open questions must
move to their owning artifact or be deleted. Roadmap entries become normative
only when promoted into requirements, spec, or a decision record.
