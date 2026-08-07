# VRS — Ontology

Canonical language for the VRS documentation system. These terms apply to VRS
trees in `context/` and package-local docs. This node has no structure layer —
the terms are largely independent, so the minimal (language-only) ontology is
enough.

## Language

**VRS**:
The documentation system composed from Vision, Requirements, and Spec documents
plus lazy companion artifacts.
_Avoid_: docs template, planning docs

**Intent Layer**:
The role VRS plays in a project: durable, structured intent that constrains
code, plans, skills, and agent behavior without replacing implementation truth.
_Avoid_: source of implementation truth, project plan

**VRS Node**:
A directory that owns a coherent VRS scope, such as a system, subsystem,
mechanism, file-kind contract, or concept.
_Avoid_: folder, module

**Child VRS Node**:
A VRS Node that refines part of a parent node's contract without restating the
parent's intent.
_Avoid_: section, nested README

**Common Subsystem**:
A recurring VRS child-node shape that is often useful across topics, such as
data model, interface, verification, integrations, or operations.
_Avoid_: companion file

**Intuition**:
The narrative entry point for a VRS Node; it gives the mental model and system
map that the formal documents assume.
_Avoid_: README

**Decision Record**:
A durable record of a consequential design, product, operational, data, or
architecture decision and the tradeoff behind it.
_Avoid_: ADR, changelog entry

**Proposed Decision**:
A PR-local working decision record that lets an agent proceed autonomously until
the decision is accepted, folded into another VRS artifact, deferred, or deleted.
_Avoid_: accepted decision, main-branch proposal

**Experiment**:
Evidence produced by this project to validate or reject an assumption, design,
performance claim, or integration behavior.
_Avoid_: plan, todo

**Reference**:
External source material a VRS Node depends on, such as API behavior, standards,
third-party documentation, or source snapshots.
_Avoid_: experiment

**Open Question**:
Unresolved design uncertainty that needs an answer before it can become
requirements, spec, decision, experiment, reference, roadmap, or nothing.
_Avoid_: roadmap item, delta, task

**Roadmap**:
Non-normative future direction that is concrete enough to preserve but not yet
part of the current VRS contract.
_Avoid_: spec, backlog, open question

**Delta**:
A confirmed, currently open divergence between the VRS contract and the current
implementation, tracked under `.delta/`.
_Avoid_: backlog, roadmap
