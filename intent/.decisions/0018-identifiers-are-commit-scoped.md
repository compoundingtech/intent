# Identifiers are commit-scoped

Status: accepted

## Context

The structured-reference experiments showed that symbolic or namespaced IDs make
Intent refactors easier than path-only references, while wiki-style links are
ambiguous in hierarchical Intent trees. The user clarified that Intent IDs do not need
to be long-term stable public identifiers; they need to be atomically consistent
within the scope of one repository commit.

## Evidence and Argument

Intent lives in Git. A commit is the reviewable unit where requirements, specs,
references, and enforcement diagnostics must agree. Treating IDs as permanent
public API would over-constrain useful Intent refactors such as splitting a
requirements file, renumbering local IDs, or moving a subsystem. Treating IDs as
purely local labels would make cross-document traceability and deterministic
checks brittle.

Commit-scoped consistency keeps the useful property: every reference in a commit
must resolve unambiguously to the current intended artifact and clause. It still
allows future commits to rename, renumber, or re-scope IDs when the Intent shape
improves, as long as all references change atomically.

## Options

| Option | Tradeoffs |
| --- | --- |
| Long-term stable IDs | Maximizes external traceability, but makes Intent restructuring unnecessarily costly. |
| Local numbering labels only | Easy to write, but weak for cross-document references and enforcement. |
| Commit-scoped IDs | Supports refactoring while preserving review-time consistency and tooling checks. |

## Decision

Intent identifiers are commit-scoped consistency handles, not long-term public API.
They may be renamed, renumbered, or re-scoped when the Intent shape improves, but a
repository commit must not contain stale, ambiguous, or mismatched references.
