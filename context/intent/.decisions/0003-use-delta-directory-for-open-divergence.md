# Use a delta directory for open divergence

Status: accepted

## Context

The Notion notes asked for a dedicated surface tracking the difference between
spec and implementation. The first draft used `implementation-delta.md`, then
the user preferred a directory for consistency with the companion-artifact
approach.

## Evidence and Argument

Open divergence behaves more like one-record-per-issue state than a narrative
document. One file per delta is easier for agents to update, assign, delete, and
review without accidentally preserving stale rows. Git history already preserves
closed records, so an archive section would duplicate history.

## Options

| Option | Tradeoffs |
| --- | --- |
| `implementation-delta.md` | Visible and simple, but tends to grow into a stale ledger. |
| `.delta/DELTA-*.md` | Consistent with companion directories and makes current open divergence easy to prune. |
| Planning-system tasks only | Avoids another Intent surface, but hides contract/reality drift from Intent readers. |

## Decision

Track confirmed Intent/implementation divergence in a lazy `.delta/` companion
directory with one file per open delta. Delta records are current-state
artifacts, not history; resolved or stale deltas are deleted after the Intent or
implementation is corrected.
