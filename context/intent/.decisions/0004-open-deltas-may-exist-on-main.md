# Open deltas may exist on main

Status: accepted

## Context

After choosing `.delta/`, Intent needed to decide whether open deltas are merge
blockers or accepted current-state documentation.

## Evidence and Argument

The user confirmed that open deltas should be allowed on main. A delta is a
truthful statement that Intent and implementation diverge; blocking all merges on
known drift would incentivize hiding or deleting useful context. The risk is
stale debt, which is handled by freshness rules rather than a blanket merge ban.

## Options

| Option | Tradeoffs |
| --- | --- |
| Ban open deltas on main | Keeps main "clean", but hides known drift or blocks useful incremental work. |
| Allow all delta records forever | Preserves context, but quickly becomes stale backlog/history. |
| Allow only confirmed open deltas | Keeps main honest while requiring stale, closed, duplicate, or speculative records to be pruned. |

## Decision

Open `.delta/` records may exist on main when they describe confirmed current
drift. Closed, stale, speculative, duplicate, or completed deltas must not
remain; they are pruned because Git history preserves the past state.
