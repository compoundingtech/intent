# VRS Ontology — Requirements

## Context

- This child node defines the contract for `ontology.md` files in VRS nodes.
- It refines [VRS-R01](../requirements.md) and [VRS-R02](../requirements.md).
- The ontology artifact is the language layer (canonical terms) plus an optional
  structure layer (how the terms relate). A flat term list is the minimal
  ontology; the language layer alone is that minimal form. The reusable
  discipline for designing the term system is the `/sk-ontology` skill; crafting
  a single term is `/sk-naming`.

## Requirements

### Must Stabilize Language

- **VRS.ONT-R01 Canonical terms:** An ontology must choose one canonical term
  when multiple names compete.
- **VRS.ONT-R02 Tight definitions:** Definitions must describe what a thing is,
  not how it is implemented.
- **VRS.ONT-R03 Avoid aliases:** Discouraged aliases must be listed when they
  are likely to appear in code, docs, or discussion.

### May Capture Structure

- **VRS.ONT-R06 Lazy structure:** An ontology may record how terms relate —
  the typed edges `isa`, `partOf`, `refines`, `dependsOn`, `related`, plus
  facets (independent axes) and grouping/layers — and the leitwort convention
  that carries a family. Structure is added only where a relationship carries
  weight; the minimal ontology is a flat term list.
- **VRS.ONT-R07 Legible membership:** When a term is a follower of an anchor,
  its name should carry the anchor's leitwort so membership is legible without
  a lookup.
- **VRS.ONT-R08 Declared inheritance:** A child ontology must name the parent
  ontology it inherits and the scope of what it adds. When a node has children
  that share terms, those shared terms belong in a root ontology at that node.

### Must Avoid Spec Drift

- **VRS.ONT-R04 No decisions:** An ontology must not carry rationale or decision
  history.
- **VRS.ONT-R05 No implementation detail:** An ontology must not become an API,
  schema, or behavior spec.
