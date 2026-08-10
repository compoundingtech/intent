# Intent Delta — Spec

This document specifies `.delta/` directories. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Directory Shape

```text
.delta/
  DELTA-001-<slug>.md
  DELTA-002-<slug>.md
```

Use the next numeric ID after scanning existing open delta files and any deleted
delta files in the same PR. Closed deltas are deleted, so IDs can become
non-contiguous; do not renumber remaining deltas just to close gaps.

Do not create `implementation-delta.md` files. A single visible delta ledger
invites stale completed work and mixed task lists; `.delta/` is the only Intent
delta shape.

## Record Shape

```markdown
# DELTA-001: <Short name>

Status: open

## Divergence

## Intent

## Implementation

## Direction

update implementation | update Intent | decide

## Resolution Signal
```

`Divergence` states the mismatch in one or two sentences. `Intent` links to the
requirement, spec clause, ontology term, or decision record. `Implementation`
links to code, config, observed behavior, or verification output. If no
implementation exists yet, this section links to the evidence that proves the
normative Intent is incomplete, contradictory, or false.

## Lifecycle

Agents must check affected deltas whenever they edit referenced Intent artifacts or
implementation surfaces.

Open deltas may exist on main. They are current-state truth about known drift,
not a merge failure by themselves.

Close a delta by deleting the file in the same change that resolves the
divergence. If the divergence was intentional, first update the relevant Intent
artifact or add a decision record; then delete the delta. Do not keep closed
deltas as historical records because Git already preserves history.
