# Grill VRS is the VRS interview procedure

Status: accepted

## Context

The user asked to recommend using VRS with a grilling procedure and suggested
the name `grill-vrs`. The meta-VRS needs to define whether that skill is a peer
documentation system, a consumer of VRS, or the operating procedure for VRS
sessions.

## Evidence and Argument

The existing procedure already acts as a Socratic interview loop that updates
VRS-like artifacts. As the meta-VRS becomes normative, keeping full artifact
rules duplicated in the skill would create drift. A `grill-vrs` name better
describes the actual job: grilling the VRS intent layer until the contract is
precise.

## Options

| Option | Tradeoffs |
| --- | --- |
| Keep the generic docs-oriented name | Avoids rename churn, but keeps a generic name and encourages duplicated doc rules. |
| Rename to `grill-vrs` and make it procedural | Aligns name with purpose and lets meta-VRS own artifact rules. |
| Fold grilling entirely into `vrs.md` | One fewer skill, but loses the explicit interview mode and one-question discipline. |

## Decision

Use `grill-vrs` as the preferred name for the Socratic interview/update
procedure that applies VRS. VRS owns artifacts and lifecycle rules; `grill-vrs`
owns the questioning, pressure-testing, validation, and inline update procedure.
The operational skill is named `grill-vrs`.
