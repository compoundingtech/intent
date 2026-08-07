# VRS — Requirements

## Context

- This topic defines VRS by using VRS. The root VRS constrains the operational
  skill and the child VRS nodes.
- Coding agents are expected to create and update VRS artifacts while working,
  so the contract must support autonomous progress without allowing temporary
  scaffolding to merge as durable truth.

## Assumptions

- **VRS-A01 Markdown as durable medium:** VRS artifacts are Markdown files
  tracked in Git.
- **VRS-A02 Lazy artifacts:** Not every VRS node needs every companion file or
  directory. Empty template structure is noise.

## Constraints

- **VRS-C01 Git-backed lifecycle:** VRS merge hygiene relies on Git review and
  history; closed decision proposals and deltas are pruned rather than archived
  in separate closed-state files.

## Acceptable Tradeoffs

- **VRS-T01 Broad decision records:** VRS uses "Decision Record" rather than
  ADR, trading some external acronym recognition for a term that covers
  non-architecture decisions.
- **VRS-T02 PR-local proposals:** Proposed decision records may exist during a
  PR, trading short-lived branch noise for agent autonomy and explicit merge
  hygiene.
- **VRS-T03 Child-node detail:** The root VRS delegates detailed file-kind rules
  to child VRS nodes, trading one-stop compactness for composability.

## Requirements

### Must Separate Artifact Responsibilities

- **VRS-R01 Clear ownership:** Every durable VRS fact must have a primary owning
  artifact kind: vision, requirements, spec, ontology, intuition, decision
  record, experiment, reference, open question, roadmap, or delta.
- **VRS-R02 No upstream restatement:** A child artifact must reference upstream
  constraints instead of restating them.
- **VRS-R03 Protected intent:** `vision.md` and `requirements.md` changes
  require deliberate confirmation because they change goals or constraints.

### Must Support Hierarchy

- **VRS-R04 Child VRS nodes:** The system must support child VRS nodes for
  subsystems, mechanisms, file-kind contracts, and cross-cutting concepts.
- **VRS-R05 Formal-first order:** The meta-VRS child-node order must begin with
  the formal VRS chain (`vision`, `requirements`, `spec`), followed by
  `ontology`, `intuition`, and companion concepts.
- **VRS-R06 Commit-scoped identifiers:** Identifiers must be internally
  consistent within each repository commit and must not depend on numeric
  directory prefixes.

### Must Keep Main Clean

- **VRS-R07 Durable decisions only on main:** Main branch must contain only
  durable accepted, deprecated, or superseded decision records.
- **VRS-R08 Proposed decisions are PR-local:** `.decisions/.proposed/` may
  exist in a PR but must be empty or absent before merge.
- **VRS-R09 Lazy companion directories:** `.experiments/` and `.reference/` are
  first-class companion directories but must be created only when they contain
  real evidence or source material.

### Must Support Verification

- **VRS-R10 Evidence promotion:** Findings from experiments or references must
  be promoted into normative VRS artifacts when they affect system truth.
- **VRS-R11 Delta tracking:** Known divergence between VRS and implementation
  must be tracked explicitly in `.delta/` rather than hidden in prose.
- **VRS-R12 Skill projection:** Operational agent skills must be reducible to
  procedures that apply the normative VRS contract instead of duplicating the
  contract.
- **VRS-R13 Delta freshness:** Delta records must be kept current and pruned
  when resolved, folded into VRS, or superseded by an accepted decision.
- **VRS-R14 Open-question freshness:** Resolved, stale, or no-longer-relevant
  questions must be removed from `open-questions.md` and moved to their owning
  artifact when the answer affects VRS.
- **VRS-R15 Roadmap separation:** Future direction that is not yet normative
  must live in `roadmap.md`, not in `spec.md`, `open-questions.md`, or
  `.delta/`.
- **VRS-R16 Referenced constraints:** Constraints caused by external systems,
  standards, APIs, platforms, or resource envelopes must be captured in
  `requirements.md` and backed by `.reference/` when the source is not obvious
  from repository code.
- **VRS-R17 Common subsystem candidates:** Data models, interfaces,
  verification loops, integrations, and operations/resource bounds must be
  considered as child VRS nodes when they are substantial enough to need their
  own requirements and spec.
- **VRS-R18 Review smells:** VRS authors and agents must check for documented
  VRS smells when creating or editing VRS artifacts and fix the artifact owner
  rather than preserving the smell.
- **VRS-R19 Drafting open questions:** During initial VRS drafting,
  `open-questions.md` may hold temporary coverage questions, but agents must try
  to resolve all open questions before finalizing unless each remaining question
  names a clear blocker.
- **VRS-R20 Assumption validation:** Load-bearing assumptions must be validated
  in the best feasible way before becoming durable VRS truth. Valid forms
  include user confirmation, research, benchmarks, e2e experiments, prototypes,
  proofs, implementation evidence, or independent agent critique.
- **VRS-R21 Evidence-backed decisions:** Durable decision records must include
  evidence or argument, principled options with tradeoffs, and a clear reason
  the chosen option is best under the current VRS context.
- **VRS-R22 Intent layer:** VRS must be described as the project's intent layer
  while keeping "VRS" as the concrete artifact/system name.
- **VRS-R23 Grill VRS procedure:** The Socratic interview/update skill should be
  named `grill-vrs` and treated as the procedure for applying the VRS contract,
  not as a duplicate source of artifact rules.
- **VRS-R24 Ontology through Grill VRS:** `grill-vrs` must handle fuzzy,
  conflicting, or overloaded language by applying the meta-VRS ontology contract.
  The reusable term-design discipline lives in the `ontology`/`naming` skills;
  `grill-vrs` owns applying it to the ontology artifact during interviews rather
  than duplicating the procedure.
- **VRS-R25 Isolated evaluation:** VRS and VRS skills must be evaluable through
  isolated scenario runs that produce evidence without modifying tracked project
  files.
- **VRS-R26 Existing VRS references:** VRS authors should inspect existing VRS
  systems as reference examples when helpful, while preserving the target
  system's own structure and constraints.
- **VRS-R27 Enforcement:** VRS must support repo hygiene enforcement through a
  narrow enforcement contract that distinguishes deterministic lint from
  semantic review without turning enforcement into the owner of artifact
  semantics.
