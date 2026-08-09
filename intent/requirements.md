# Intent — Requirements

## Context

- This topic defines Intent by using Intent. The root Intent constrains the operational
  skill and the child Intent nodes.
- Coding agents are expected to create and update Intent artifacts while working,
  so the contract must support autonomous progress without allowing temporary
  scaffolding to merge as durable truth.

## Assumptions

- **INTENT-A01 Markdown as durable medium:** Intent artifacts are Markdown files
  tracked in Git.
- **INTENT-A02 Lazy artifacts:** Not every Intent node needs every companion file or
  directory. Empty template structure is noise.

## Constraints

- **INTENT-C01 Git-backed lifecycle:** Intent merge hygiene relies on Git review and
  history; closed decision proposals and deltas are pruned rather than archived
  in separate closed-state files.

## Acceptable Tradeoffs

- **INTENT-T01 Broad decision records:** Intent uses "Decision Record" rather than
  ADR, trading some external acronym recognition for a term that covers
  non-architecture decisions.
- **INTENT-T02 PR-local proposals:** Proposed decision records may exist during a
  PR, trading short-lived branch noise for agent autonomy and explicit merge
  hygiene.
- **INTENT-T03 Child-node detail:** The root Intent delegates detailed file-kind rules
  to child Intent nodes, trading one-stop compactness for composability.

## Requirements

### Must Separate Artifact Responsibilities

- **INTENT-R01 Clear ownership:** Every durable Intent fact must have a primary owning
  artifact kind: vision, requirements, spec, ontology, intuition, decision
  record, experiment, reference, open question, roadmap, or delta.
- **INTENT-R02 No upstream restatement:** A child artifact must reference upstream
  constraints instead of restating them.
- **INTENT-R03 Protected intent:** `vision.md` and `requirements.md` changes
  require deliberate confirmation because they change goals or constraints.

### Must Support Hierarchy

- **INTENT-R04 Child Intent nodes:** The system must support child Intent nodes for
  subsystems, mechanisms, file-kind contracts, and cross-cutting concepts.
- **INTENT-R05 Formal-first order:** The root Intent contract's child-node order must begin with
  the formal Intent chain (`vision`, `requirements`, `spec`), followed by
  `ontology`, `intuition`, and companion concepts.
- **INTENT-R06 Commit-scoped identifiers:** Identifiers must be internally
  consistent within each repository commit and must not depend on numeric
  directory prefixes.

### Must Keep Main Clean

- **INTENT-R07 Durable decisions only on main:** Main branch must contain only
  durable accepted, deprecated, or superseded decision records.
- **INTENT-R08 Proposed decisions are PR-local:** `.decisions/.proposed/` may
  exist in a PR but must be empty or absent before merge.
- **INTENT-R09 Lazy companion directories:** `.experiments/` and `.reference/` are
  first-class companion directories but must be created only when they contain
  real evidence or source material.

### Must Support Verification

- **INTENT-R10 Evidence promotion:** Findings from experiments or references must
  be promoted into normative Intent artifacts when they affect system truth.
- **INTENT-R11 Delta tracking:** Known divergence between Intent and implementation
  must be tracked explicitly in `.delta/` rather than hidden in prose.
- **INTENT-R12 Skill projection:** Operational agent skills must be reducible to
  procedures that apply the normative Intent contract instead of duplicating the
  contract.
- **INTENT-R13 Delta freshness:** Delta records must be kept current and pruned
  when resolved, folded into Intent, or superseded by an accepted decision.
- **INTENT-R14 Open-question freshness:** Resolved, stale, or no-longer-relevant
  questions must be removed from `open-questions.md` and moved to their owning
  artifact when the answer affects Intent.
- **INTENT-R15 Roadmap separation:** Future direction that is not yet normative
  must live in `roadmap.md`, not in `spec.md`, `open-questions.md`, or
  `.delta/`.
- **INTENT-R16 Referenced constraints:** Constraints caused by external systems,
  standards, APIs, platforms, or resource envelopes must be captured in
  `requirements.md` and backed by `.reference/` when the source is not obvious
  from repository code.
- **INTENT-R17 Common subsystem candidates:** Data models, interfaces,
  verification loops, integrations, and operations/resource bounds must be
  considered as child Intent nodes when they are substantial enough to need their
  own requirements and spec.
- **INTENT-R18 Review smells:** Intent authors and agents must check for documented
  Intent smells when creating or editing Intent artifacts and fix the artifact owner
  rather than preserving the smell.
- **INTENT-R19 Drafting open questions:** During initial Intent drafting,
  `open-questions.md` may hold temporary coverage questions, but agents must try
  to resolve all open questions before finalizing unless each remaining question
  names a clear blocker.
- **INTENT-R20 Assumption validation:** Load-bearing assumptions must be validated
  in the best feasible way before becoming durable Intent truth. Valid forms
  include user confirmation, research, benchmarks, e2e experiments, prototypes,
  proofs, implementation evidence, or independent agent critique.
- **INTENT-R21 Evidence-backed decisions:** Durable decision records must include
  evidence or argument, principled options with tradeoffs, and a clear reason
  the chosen option is best under the current Intent context.
- **INTENT-R22 One canonical name:** Intent is the only name for the project's
  intent layer, artifact system, corpus, nodes, tooling contracts, and
  operational procedures. Paths and command names use `intent`; structured IDs
  and diagnostic namespaces use `INTENT`.
- **INTENT-R23 Grill Intent procedure:** The Socratic interview/update skill should be
  named `grill-intent` and treated as the procedure for applying the Intent contract,
  not as a duplicate source of artifact rules.
- **INTENT-R24 Ontology through Grill Intent:** `grill-intent` must handle fuzzy,
  conflicting, or overloaded language by applying the root Intent ontology contract.
  The reusable term-design discipline lives in the `ontology`/`naming` skills;
  `grill-intent` owns applying it to the ontology artifact during interviews rather
  than duplicating the procedure.
- **INTENT-R25 Isolated evaluation:** Intent and Intent skills must be evaluable through
  isolated scenario runs that produce evidence without modifying tracked project
  files.
- **INTENT-R26 Existing Intent references:** Intent authors should inspect existing Intent
  systems as reference examples when helpful, while preserving the target
  system's own structure and constraints.
- **INTENT-R27 Enforcement:** Intent must support repo hygiene enforcement through a
  narrow enforcement contract that distinguishes deterministic lint from
  semantic review without turning enforcement into the owner of artifact
  semantics.
