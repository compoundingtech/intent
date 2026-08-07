# Proposal exit is enforced at merge, not relaxed into a durable state

Status: accepted

## Context

`VRS.DEC-R10` confines proposed decision records to `.decisions/.proposed/`
"during a PR only", and `VRS.DEC-R11` requires every proposed record to be
accepted, folded into another artifact, moved to `open-questions.md`, or deleted
**before merge**.

Practice has drifted from both. The question raised was whether proposals should
instead become a durable state with a cross-workstream ratification backlog — a
queue the principal works through — which would mean amending R10 and R11 rather
than enforcing them.

## Evidence and Argument

Measured across `context/` on 2026-07-30: **six live proposed records, dated
2026-07-20 to 2026-07-29**, on a branch that merged repeatedly during that window.
R11 is therefore already violated rather than merely at risk — proposals are
surviving merges today, and none of the six has been ratified, folded, or demoted.

One is also at the wrong path (`04-agent-context/.proposed/` rather than
`.decisions/.proposed/`), which is the shape drift that follows once a rule is
known not to be checked.

That measurement cuts against legitimising the backlog. The six records are not
evidence that a durable proposed state is *working*; they are evidence of what an
unenforced one becomes. Ten days unratified is already the graveyard failure the
durable-state option would have had to design against, observed before the option
was adopted.

The principal's stated direction is that decisions touching VRS should route
through the proposal mechanism with an explicit ratification step — which the
existing contract already provides. What was missing was enforcement, not a new
state.

## Options

| Option | Tradeoffs |
| --- | --- |
| Enforce R11 with a merge gate | Contract unchanged; kills the graveyard by construction. The cheapest way to a green gate is `rm`, which discards the reasoning the proposal recorded |
| Make proposals durable, backlog-tracked | Legitimises current practice and gives ratification a queue; requires amending protected requirements, and the observed six records show the state becomes where decisions go to die |
| Durable but expiry-bounded | Keeps the thinking without a graveyard; most machinery — a review-by date, a surfacing mechanism, and something that runs it |
| Leave unenforced | No work; the rule stays known-not-checked, and shape drift continues |

## Decision

**R10 and R11 stand as written, and are enforced at merge.** A check fails when
any `.decisions/.proposed/` directory is non-empty, so each proposal must reach
one of R11's four exits before it can land.

The six existing records are the immediate backlog to clear under that rule.

Ratification is a conversation, not a gate outcome. **The chief-of-staff persona
raises open proposals** — surfacing them is decision-preparation, which is what
that persona is for — but the ratification itself commonly happens directly
between the principal and the owning workstream's orchestrator, because that is
where the context sits. The gate forces the exit to happen; it does not decide
which exit, and it is not the mechanism by which consent is obtained.

## Consequences

**The `rm`-to-green failure mode is accepted with eyes open, and is the residual
risk.** A gate on emptiness cannot distinguish a considered deletion from a
deletion taken to unblock a merge, and deleting the file discards the reasoning
the proposal existed to record. Two mitigations are available and neither is
adopted here: requiring the exit to be *stated* rather than inferred from
absence, and demoting-to-`open-questions.md` as the default exit rather than
deletion. If the backlog reappears as repeated deletions, that is the signal to
revisit — the metric is proposals deleted without a corresponding decision,
open question, or fold.

Enforcement makes the wrong-path record (`04-agent-context/.proposed/`) fail
until it moves under a `.decisions/` parent, which is the intended side effect:
an unchecked shape rule drifts.

A proposal that genuinely needs to outlive its PR has an exit for that already —
`open-questions.md`, which is designed to hold unresolved design uncertainty and
carries a blocker. Reaching for it is not a workaround; it is the contract's
answer to "not ready to ratify".
