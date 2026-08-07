# VRS Reference — Requirements

## Context

- This child node defines the contract for `.reference/` companion directories.
- It refines [VRS-R09](../requirements.md) and [VRS-R10](../requirements.md).

## Requirements

### Must Preserve External Context

- **VRS.REF-R01 External source material:** References must capture external
  APIs, standards, behavior notes, source snapshots, or integration assumptions.
- **VRS.REF-R02 Source identity:** References must identify where the material
  came from and when it was captured when freshness matters.
- **VRS.REF-R03 Relevance:** References must state which VRS assumption,
  requirement, spec clause, or decision they support.
- **VRS.REF-R04 Constraint source:** External constraints in `requirements.md`
  must cite the reference material that establishes or explains the constraint.

### Must Avoid Becoming Normative

- **VRS.REF-R05 Lazy directory:** `.reference/` exists only when it contains
  real source material.
- **VRS.REF-R06 Promotion required:** VRS truth derived from a reference must be
  promoted into a normative artifact.
