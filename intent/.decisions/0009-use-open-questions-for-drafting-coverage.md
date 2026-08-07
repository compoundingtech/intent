# Use open questions for drafting coverage

Status: accepted

## Context

The Notion notes asked for a drafting todo list so agents do not lose coverage
while going deep on one branch. The user preferred using `open-questions.md`
temporarily for that purpose, with a rule that open questions should be resolved
unless clearly blocked.

## Evidence and Argument

Initial VRS drafting naturally produces uncertainty and unexplored areas. A
separate checklist artifact would add another file type that must later be
cleaned up. `open-questions.md` already has lifecycle rules and can absorb
temporary drafting coverage if agents apply resolution pressure before
finalizing.

## Options

| Option | Tradeoffs |
| --- | --- |
| Add `drafting-checklist.md` | Explicit coverage list, but creates another temporary artifact that may merge stale. |
| Keep checklist only in agent plan | Lightweight, but loses durable handoff if the session ends. |
| Use `open-questions.md` with blockers | Reuses an existing artifact and preserves coverage while requiring cleanup. |

## Decision

Initial VRS drafting may use `open-questions.md` as the temporary coverage
surface. Before finalizing VRS work, agents must try to resolve every question;
any remaining question must name the blocker that prevents resolution.
