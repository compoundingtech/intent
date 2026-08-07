# Extract to a fresh repository with a fresh init, not filtered history

Status: accepted

## Context

This corpus was authored inside `schickling/dotfiles`, a private repository, and
now lives in `compoundingtech/intent`, which is destined to become public. The
extraction moved two things at once:

- `context/vrs/` (106 files) became this repository's `intent/`, verbatim, with
  section names and internal layout unchanged.
- `context/coding-agents/14-axe/12-vrs/` (5 files) became `context/cli/` — the
  contract for the checker that enforces this corpus, which is a consumer of the
  methodology rather than part of it, and so sits outside `intent/`.

Both were taken from `schickling/dotfiles` at commit
`1b246bed551e520975e06ccdbb998f72ec8a5d9d`.

Git history could not come with them unchanged. The question this record settles
is what happened to it, because the answer is not recoverable by inspecting the
result: a repository whose first commit contains 111 files looks the same whether
its history was discarded deliberately or lost by accident.

## Evidence and Argument

The history that would have been carried over, measured against the source commit:

| What | Path set | Commits |
| --- | --- | --- |
| The corpus | `context/vrs` | 57 |
| The corpus plus the checker source | `context/vrs`, `flakes/axe/src/vrs.rs` | 64 |
| The CLI subsystem contract | `context/coding-agents/14-axe/12-vrs` | 24 |

Those spans run **2026-06-18 to 2026-07-30** by committer date. Author dates begin
one day earlier, 2026-06-17 — the first two corpus commits were authored that
afternoon and committed the following morning. The committer range is the one used
here; the distinction is recorded because the two conventions disagree at the
boundary and a reader reproducing the numbers should know which was meant.

The decisive property is that those commit messages were written for a private
audience. They reference internal hosts, sibling subsystems, operational
incidents, and in-flight work that has no meaning outside the originating
repository — and they were never written with the expectation of being read by
anyone outside it. Filtering paths with a history-rewriting tool carries the file
history across, but it carries every one of those messages with it, and reviewing
64 commit messages for disclosure is a manual, unverifiable, one-way operation.
The failure mode is asymmetric: a message that should have been redacted and was
not cannot be recalled once the repository is public, whereas history that was
never exported can always be exported later.

Nothing is destroyed by this choice. `schickling/dotfiles` is not being rewritten,
so the full history — every commit, message, and intermediate state — remains
intact there and available to anyone with access to that repository. Only the
*copy* in this repository starts fresh. The provenance that a reader actually
needs from history is the origin, the source commit, and the shape of what moved,
and a record states those more legibly than 64 commits do.

## Options

| Option | Tradeoffs |
| --- | --- |
| Fresh init, provenance recorded in this record | No private commit message can leak, and the property is verifiable by inspection rather than by trusting a review pass; per-file authorship and the reasoning of intermediate commits are not available in this repository, and `git blame` starts here |
| Filtered history via a path-rewriting tool | Preserves per-file authorship, blame, and the incremental reasoning of 64 commits; requires manually reviewing every one of those messages for content written for a private audience, which is unverifiable, easy to get wrong, and irreversible once the repository is public |
| Fresh init with no provenance record | Cheapest and equally leak-proof; leaves a repository that cannot answer where it came from or what its history was, which is precisely the silent-rot failure this corpus exists to prevent |
| Squashed import of all 64 commits into one | Keeps a single import commit as a marker; the concatenated messages carry the same private content as the filtered option with none of the blame benefit |

## Decision

**The corpus was extracted by fresh `git init`. The history in this repository
begins with the seed commit, and this record carries the provenance instead.**

Concretely, and for the record:

- **Origin:** `schickling/dotfiles`, private.
- **Source commit:** `1b246bed551e520975e06ccdbb998f72ec8a5d9d`.
- **Corpus history at that commit:** 57 commits touching `context/vrs`, or 64
  counting the checker source alongside it, spanning 2026-06-18 to 2026-07-30 by
  committer date.
- **Full history remains in `schickling/dotfiles`** and is available there to
  anyone with access to that repository. It was not rewritten, and nothing was
  discarded — only not copied.
- **`context/coding-agents/14-axe/12-vrs/` moved in the same extraction**, to
  `context/cli/`, and its own 24 commits are covered by the same reasoning.

## Consequences

`git blame` and `git log` in this repository answer questions about this
repository only. Anyone tracing why a line of this corpus reads as it does must
go to `schickling/dotfiles` at or before the source commit, which requires access
to a private repository. That cost falls hardest on exactly the readers this
repository is being opened for, and it is accepted: the durable reasoning is
supposed to live in `.decisions/` and `.delta/` records rather than in commit
messages, and this corpus has 35 of the former. If that proves insufficient in
practice, the remedy is to improve the records, not to export the history.

Two consequences of the move itself are recorded here because they are otherwise
invisible in the result:

**The CLI subsystem lost its parent edge.** In the originating repository,
`14-axe/12-vrs/requirements.md` built on `14-axe/requirements.md`, the contract of
the host tool it was a subsystem of. That parent did not move and does not exist
here, so the edge was removed rather than rewritten to prose. Keeping any form of
it would have asserted a relationship that is no longer true while making it
unverifiable — the precise failure this corpus exists to prevent. `context/cli/`
is a root node in this repository. If it should later refine something here, that
is a deliberate authoring decision to be taken on its own merits.

**Six citations that pointed outside the corpus were rewritten.** They named
records in `context/coding-agents/` and `context/observability/` that remain
private, plus one absolute URL into `schickling/dotfiles` that returns 404 for
anyone without access. Rather than invent public URLs that do not exist, the
citations were converted to plain backticked paths qualified by the originating
repository: the reference stays precise and attributable, and a reader with access
can resolve it, but nothing pretends to be a working link. The single citation that
pointed at the CLI subsystem became an internal link, since its target moved into
this repository.
