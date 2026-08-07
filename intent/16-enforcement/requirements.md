# VRS Enforcement — Requirements

## Context

- This child node defines how VRS hygiene is checked in real repository VRS
  trees.
- It refines [VRS-R18](../requirements.md), [VRS-R25](../requirements.md), and
  [VRS-R27](../requirements.md).

## Requirements

### Must Preserve Semantic Ownership

- **VRS.ENF-R01 Enforcement does not own semantics:** Enforcement must validate
  VRS artifacts against the contracts owned by the file-kind and concept nodes;
  it must not become the place where those contracts are defined.
- **VRS.ENF-R02 Narrow scope:** Enforcement covers repo hygiene, diagnostics,
  and review gates. It does not own artifact authoring procedure, isolated
  scenario evaluation, or implementation of every future VRS-related tool.

### Must Separate Mechanical and Semantic Checks

- **VRS.ENF-R03 Deterministic checks:** Mechanical invariants should be checked
  by token-free deterministic tooling when feasible.
- **VRS.ENF-R04 Semantic review:** Intent-quality checks that require judgment
  must be reported separately from deterministic lint findings.
- **VRS.ENF-R05 Blocking clarity:** Each enforcement mode must say whether it is
  merge-blocking, advisory, or review-only.

### Must Produce Actionable Diagnostics

- **VRS.ENF-R06 Artifact owner:** Every finding must name the VRS artifact or
  child node that owns the fix.
- **VRS.ENF-R07 Evidence:** Every finding must include enough evidence for a
  reader or agent to reproduce or inspect it.
- **VRS.ENF-R08 Machine-readable output:** Tooling-oriented enforcement should
  support structured output so agents and CI can route findings precisely.

### Must Fit Existing Workflows

- **VRS.ENF-R09 Local-first:** Deterministic enforcement must be runnable locally
  before CI.
- **VRS.ENF-R10 Eval feedback:** False positives, missing checks, or ambiguous
  diagnostics should feed back into [15-evaluation](../15-evaluation/spec.md)
  scenarios or the owning VRS contract.

### Must Enforce Cross-References Incrementally

- **VRS.ENF-R11 Xref subset:** Cross-reference enforcement must start with a
  deterministic subset that can be checked from repository contents alone.
- **VRS.ENF-R12 Commit consistency:** Xref enforcement must validate references
  against the current commit, not require identifiers to remain stable across
  future commits.
- **VRS.ENF-R13 Wiki-link migration:** Wiki-style links in normative artifacts
  should be warned on first and promoted to errors only after the intended
  migration path is clear.
