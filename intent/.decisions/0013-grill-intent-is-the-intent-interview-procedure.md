# Grill Intent is the Intent interview procedure

Status: accepted

## Context

The user asked to recommend using Intent with a grilling procedure and suggested
the name `grill-intent`. The root Intent contract needs to define whether that skill is a peer
documentation system, a consumer of Intent, or the operating procedure for Intent
sessions.

## Evidence and Argument

The existing procedure already acts as a Socratic interview loop that updates
Intent-like artifacts. As the root Intent contract becomes normative, keeping full artifact
rules duplicated in the skill would create drift. A `grill-intent` name better
describes the actual job: grilling the Intent layer until the contract is
precise.

## Options

| Option | Tradeoffs |
| --- | --- |
| Keep the generic docs-oriented name | Avoids rename churn, but keeps a generic name and encourages duplicated doc rules. |
| Rename to `grill-intent` and make it procedural | Aligns name with purpose and lets root Intent contract own artifact rules. |
| Fold grilling entirely into `intent.md` | One fewer skill, but loses the explicit interview mode and one-question discipline. |

## Decision

Use `grill-intent` as the preferred name for the Socratic interview/update
procedure that applies Intent. Intent owns artifacts and lifecycle rules; `grill-intent`
owns the questioning, pressure-testing, validation, and inline update procedure.
The operational skill is named `grill-intent`.
