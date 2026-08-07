# DELTA-001: Proposal exit is unenforced and already violated

Status: open

## Divergence

`VRS.DEC-R10` confines proposed decision records to `.decisions/.proposed/`
"during a PR only". `VRS.DEC-R11` requires every proposed record to be accepted,
folded into another artifact, moved to `open-questions.md`, or deleted **before
merge**. [0034](../.decisions/0034-proposal-exit-is-enforced-at-merge-not-relaxed.md)
decides those requirements stand and are **enforced at merge**.

Neither the enforcement nor the requirement holds today. Measured 2026-07-30:

```
live proposed records                     6
oldest                          2026-07-20   (10 days)
merges to the branch in that window       many
check enforcing R11                      none
```

One record is also at the wrong path — `04-agent-context/.proposed/` rather than
`04-agent-context/.decisions/.proposed/` — which is the shape drift that follows a
rule known not to be checked.

The six are not a backlog awaiting ratification; under R11 each is a record that
should already have exited. 0034 treats them as evidence of what an unenforced
proposed state becomes, and they are the immediate work to clear.

## VRS

- [0034](../.decisions/0034-proposal-exit-is-enforced-at-merge-not-relaxed.md) — the
  decision, the rejected durable-state option, and the accepted `rm`-to-green risk.
- `VRS.DEC-R10`, `VRS.DEC-R11` in [06-decisions/requirements.md](../06-decisions/requirements.md).
- [16-enforcement](../16-enforcement/) — where a check belongs.

## Implementation

- No check asserts `.decisions/.proposed/` is empty. Adding one is the whole of the
  enforcement.
- The six live records, each needing one of R11's four exits:
  - `coding-agents/.decisions/.proposed/incubate-agent-spec-lifecycle-as-an-eval-family.md`
  - `coding-agents/.decisions/.proposed/layer-dynamic-agent-spec-lifecycle-ownership.md`
  - `coding-agents/.decisions/.proposed/separate-agent-spec-publication-from-explicit-replacement.md`
  - `coding-agents/.decisions/.proposed/use-an-immutable-content-addressed-agent-spec-catalog.md`
  - `coding-agents/04-agent-context/.proposed/arrival-obligations-for-lineage-and-vocabulary.md`
    (also at the wrong path)
  - `observability/.decisions/.proposed/per-language-emission-binding-layer.md`

## Direction

update implementation

## Resolution Signal

A check fails when any `.decisions/.proposed/` is non-empty, and the six existing
records have each reached a stated exit. Closed when a proposal cannot survive a
merge silently.

Watch the failure mode 0034 accepted: if the backlog clears mainly through
deletion, the enforcement has converted reasoning into green rather than into
decisions. The metric is proposals deleted without a corresponding decision, open
question, or fold — and clearing these six is the first sample of it.

Ratification itself is not this delta's concern: it is raised by the
chief-of-staff and usually settled directly with the owning workstream's
orchestrator, where the context sits.
