# VRS Common Subsystems — Spec

This document specifies common subsystem candidates in VRS trees. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Candidate Subsystems

Common subsystem nodes use normal numeric child-node naming:

```text
<vrs-node>/
  01-data-model/
    requirements.md
    spec.md
  02-interface/
    requirements.md
    spec.md
  03-verification/
    requirements.md
    spec.md
  04-integrations/
    requirements.md
    spec.md
  05-operations/
    requirements.md
    spec.md
```

These names are examples, not fixed global numbering. Choose names and order
that match the topic dependency structure.

## Data Model

Create a data-model subsystem when entities, schemas, migrations, ownership,
lifecycle, serialization, storage, or compatibility are central enough to need a
separate contract.

The data-model spec should make concrete:

- entity/schema definitions;
- identity and ownership rules;
- lifecycle/state transitions;
- migration and compatibility rules;
- persistence or serialization formats.

## Interface

Create an interface subsystem when user/system surfaces need stable contracts:
CLI commands, config files, APIs, UI routes, event payloads, or environment
variables.

Specify the audience for each surface and the concrete shape users or systems
interact with.

## Verification

Create a verification subsystem when testing architecture, conformance,
benchmarks, e2e validation, or feedback loops are part of the system design.

Verification specs should state what proves each important requirement and what
kind of evidence belongs in `.experiments/`.

## Integrations

Create an integrations subsystem when external systems impose assumptions,
constraints, auth models, API limits, compatibility behavior, or freshness
requirements.

Integration constraints should cite `.reference/` records when the source is
external.

## Operations

Create an operations subsystem when runtime behavior needs explicit design:
observability, rollout, recovery, incident handling, capacity, disk, memory,
CPU, latency, or cost bounds.

Operational bounds that constrain the system belong in `requirements.md`
constraints; mechanisms for satisfying them belong in the operations spec.
