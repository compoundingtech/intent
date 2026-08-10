# Intent Enforcement — Requirements

## Context

- This child node defines how Intent hygiene is checked in real repository Intent
  trees.
- It refines [INTENT-R18](../requirements.md), [INTENT-R25](../requirements.md), and
  [INTENT-R27](../requirements.md).

## Requirements

### Must Preserve Semantic Ownership

- **INTENT.ENF-R01 Enforcement does not own semantics:** Enforcement must validate
  Intent artifacts against the contracts owned by the file-kind and concept nodes;
  it must not become the place where those contracts are defined.
- **INTENT.ENF-R02 Narrow scope:** Enforcement covers repo hygiene, diagnostics,
  and review gates. It does not own artifact authoring procedure, isolated
  scenario evaluation, or implementation of every future Intent-related tool.

### Must Separate Mechanical and Semantic Checks

- **INTENT.ENF-R03 Deterministic checks:** Mechanical invariants should be checked
  by token-free deterministic tooling when feasible.
- **INTENT.ENF-R04 Semantic review:** Intent-quality checks that require judgment
  must be reported separately from deterministic lint findings.
- **INTENT.ENF-R05 Blocking clarity:** Each enforcement mode must say whether it is
  merge-blocking, advisory, or review-only.

### Must Produce Actionable Diagnostics

- **INTENT.ENF-R06 Artifact owner:** Every finding must name the Intent artifact or
  child node that owns the fix.
- **INTENT.ENF-R07 Evidence:** Every finding must include enough evidence for a
  reader or agent to reproduce or inspect it.
- **INTENT.ENF-R08 Machine-readable output:** Tooling-oriented enforcement should
  support structured output so agents and CI can route findings precisely.

### Must Fit Existing Workflows

- **INTENT.ENF-R09 Local-first:** Deterministic enforcement must be runnable locally
  before CI.
- **INTENT.ENF-R10 Eval feedback:** False positives, missing checks, or ambiguous
  diagnostics should feed back into [15-evaluation](../15-evaluation/spec.md)
  scenarios or the owning Intent contract.

### Must Enforce Cross-References Incrementally

- **INTENT.ENF-R11 Xref subset:** Cross-reference enforcement must start with a
  deterministic subset that can be checked from repository contents alone.
- **INTENT.ENF-R12 Commit consistency:** Xref enforcement must validate references
  against the current commit, not require identifiers to remain stable across
  future commits.
- **INTENT.ENF-R13 Wiki-link migration:** Wiki-style links in normative artifacts
  should be warned on first and promoted to errors only after the intended
  migration path is clear.
