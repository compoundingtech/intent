# Use Intent as the only layer and system name

Status: accepted

## Context

The repository, corpus directory, checker crate, binary, and operational skill
already use the name Intent, while the normative corpus and machine contracts
retained a second concrete system name. The two-name model made it unclear
whether Intent named the repository, the conceptual layer, or the artifact
system itself.

The repository owner approved completing the rename so one name covers the
layer, corpus, artifacts, checker, and public contracts.

## Evidence and Argument

Before the migration, the retired name appeared 1,298 times across 113 of 125
tracked files and in 13 tracked paths. It was embedded in prose, requirement and
rule IDs, JSON schema versions, Rust public types, accepted document headings,
tests, fixtures, and CI assertions. Keeping compatibility spellings would leave
the ambiguity in precisely the machine-readable surfaces where a canonical name
matters most.

Intent already names the repository, `intent/` corpus, `intent` CLI, crate, and
skill. Using it for the documentation system as well removes a translation rule
without changing the artifact model.

## Options

| Option | Tradeoffs |
| --- | --- |
| Keep two names | Avoids a breaking migration, but preserves ambiguity in prose and machine contracts. |
| Rename prose only | Improves onboarding, but leaves code, paths, rules, schemas, and fixtures teaching the retired name. |
| Use Intent everywhere | Creates one coordinated breaking change and one canonical vocabulary across every surface. |

## Decision

Use **Intent** as the only name for the project's intent layer, documentation
system, corpus, artifacts, nodes, checker contracts, and operational procedures.
Use lowercase `intent` for filesystem paths, command names, crate/package names,
and schema namespace components; use uppercase `INTENT` in structured IDs and
diagnostic rule namespaces.

Rename all affected surfaces atomically. Do not retain compatibility aliases in
this repository, because an alias would preserve the two-name model. Downstream
consumers must migrate their command names, schema-version checks, rule IDs, and
document grammar in the same release boundary.
