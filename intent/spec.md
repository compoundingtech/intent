# VRS — Spec

This document specifies the VRS documentation system itself. It is the root
contract for the operational VRS skill and for VRS trees in `context/` or
package-local docs.

## Status

Draft.

## Scope

This root spec defines the shape of VRS nodes, companion artifacts, hierarchy,
and lifecycle rules that apply across VRS documents.

It does not define the full section-level contract for every file kind. Those
contracts live in child VRS nodes so the root remains small and composable.

VRS is the project's **intent layer**. It records durable, structured intent
that constrains code, plans, skills, and agent behavior without replacing
implementation truth. "Intent layer" names the role; VRS remains the concrete
artifact system.

## Root Node Shape

A VRS node is a directory whose files collectively describe one durable system,
subsystem, concept, or documentation contract.

```text
<vrs-node>/
  vision.md                    # protected: why and success criteria
  requirements.md              # protected: testable constraints
  spec.md                      # living blueprint
  ontology.md                  # canonical terms
  intuition.md                 # narrative entry point and mental model
  open-questions.md            # unresolved design questions
  roadmap.md                   # non-committed future direction
  .delta/
    DELTA-001-<slug>.md        # tracked contract/reality divergence
  .decisions/
    0001-<slug>.md             # durable decision record
    .proposed/
      <slug>.md                # PR-local only; must not merge to main
  .experiments/
    <slug>.md                  # validation evidence
  .reference/
    <slug>.md                  # external source snapshot or integration note
```

All files and companion directories are lazy. A VRS node creates only the
artifacts that carry real content.

## Entry Point

`intuition.md` is the narrative entry point. It replaces the VRS use of
`README.md`, but it follows the formal VRS chain and ontology in the directory
shape. It gives the reader the system map, concepts, and operating model that
the formal documents assume.

Repository `README.md` files may still exist for package-level onboarding, but
they are not VRS artifacts.

## Child VRS Nodes

The meta-VRS is self-recursive: each major VRS file kind or concept can have a
child VRS node that defines its own contract.

Initial child nodes:

| Node | Owns |
| --- | --- |
| `01-vision/` | Problem framing, goals, exclusions, success criteria |
| `02-requirements/` | Assumptions, constraints, tradeoffs, requirement IDs |
| `03-spec/` | Implementation blueprint, diagrams, design questions |
| `04-ontology/` | Canonical terminology, term structure, and ambiguity handling |
| `05-intuition/` | Narrative entry point, system map, and mental-model rules |
| `06-decisions/` | Durable decisions and PR-local proposed decisions |
| `07-experiments/` | Validation evidence, prototypes, benchmarks, research |
| `08-reference/` | External system assumptions and source snapshots |
| `09-delta/` | Gap tracking between VRS and implementation |
| `10-open-questions/` | Unresolved design uncertainty |
| `11-roadmap/` | Future direction that is not yet contract |
| `12-common-subsystems/` | Common subsystem patterns worth considering |
| `13-review-smells/` | VRS smells and bad patterns agents must check |
| `14-grill-vrs/` | Interview procedure for interrogating and updating VRS |
| `15-evaluation/` | Evaluation approach for VRS and VRS skills |
| `16-enforcement/` | Repo hygiene checks, diagnostics, and review gates |

Child nodes use numeric prefixes for reading order only. The order starts with
the formal VRS chain (`vision` -> `requirements` -> `spec`), then language
(`ontology`), then narrative entry (`intuition`), then companion concepts.
Stable identifiers use semantic namespaces, not directory numbers.

## Decision Records

Decision records live under `.decisions/` and are named
`0001-<slug>.md`, `0002-<slug>.md`, and so on. They record consequential
decisions, not only architecture decisions.

Use a decision record when all are true:

- the choice is costly or confusing to reverse;
- a future reader would wonder why the system is shaped this way;
- the choice reflects a real tradeoff between plausible alternatives.

Accepted decision records include at least one applicable evidence or argument
type, principled options with tradeoffs, and a clear explanation for why the
chosen option is best under the current VRS context and constraints. Important
or risky decisions should use multiple independent evidence forms when feasible,
such as research plus benchmark, prototype plus user confirmation, or proof plus
implementation evidence. If the options or evidence are not clear yet, keep the
record proposed or move the uncertainty to `open-questions.md`.

Accepted, deprecated, and superseded decision records may exist on main.
Proposed decision records are branch-local scaffolding only:

- proposed records MAY live under `.decisions/.proposed/` while a PR is open;
- `.decisions/.proposed/` MUST NOT be merged to main;
- before merge, every proposed record is accepted into `.decisions/`, folded
  into `requirements.md` or `spec.md`, moved to `open-questions.md`, or deleted.

## Experiments and Reference

`.experiments/` and `.reference/` are first-class companion directories, but
they are lazy. Most VRS nodes do not need both.

Use `.experiments/` for validation evidence produced by this project:
prototypes, benchmarks, e2e validation, proofs, user confirmations, independent
critique, failed approaches, and research reports whose result informs the VRS
contract.

Use `.reference/` for external source material the VRS depends on: API
snapshots, third-party behavior notes, standards excerpts, or integration
assumptions.

Accepted findings from experiments and references move into the normative
documents as requirements, spec clauses, ontology terms, or decision records.
The companion directories preserve the evidence trail, not the source of truth.

## Delta

`.delta/` is a first-class companion directory for confirmed divergence between
the VRS contract and current implementation, observed behavior, or verification
evidence. It is lazy and visible in normal review because it answers which parts
of the contract are not true yet.

Each open delta is one file:

```text
.delta/
  DELTA-001-<slug>.md
```

A delta is not a task, backlog item, or historical record. It must stay current:

- create a delta only after confirming a real contract/reality divergence;
- allow open deltas on main when they represent current known drift;
- allow non-contiguous `DELTA-*` IDs after closed deltas are deleted;
- update it whenever the relevant VRS or implementation changes;
- close it by deleting the file once implementation and VRS agree again;
- if the divergence was intentional, replace the delta with the relevant
  requirement, spec change, or decision record;
- stale, vague, or completed deltas must be pruned before merge.

## Open Questions

`open-questions.md` tracks unresolved design uncertainty that is not yet a
decision, requirement, spec clause, experiment result, roadmap item, or confirmed
`.delta` record.

Resolved questions must leave `open-questions.md`. Resolution moves the content
to the artifact that now owns it: spec, requirements, ontology, decision record,
experiment, reference, roadmap, or deletion if it no longer matters.

During initial VRS drafting, `open-questions.md` may also hold temporary
coverage questions so important areas are not forgotten. Before finalizing the
draft, agents must try to resolve every open question; any remaining question
must name the blocker that prevents resolution.

## Roadmap

`roadmap.md` is an optional companion file for future direction that is too
large or too far out to become current spec, but concrete enough to preserve.
It is non-normative: roadmap entries do not constrain implementation until they
are promoted into requirements, spec, or a decision record.

Use roadmap for future capabilities, later phases, deferred integrations, or
possible product/system directions. Do not use it for known current drift
(`.delta/`), unresolved design uncertainty (`open-questions.md`), or active
implementation tasks.

## Common Subsystems

Some recurring design areas are often large enough to deserve child VRS nodes,
but they are not generic companion files. Create them when the topic has real
surface area, constraints, or independent verification needs.

Common candidates:

| Candidate | Use when |
| --- | --- |
| `NN-data-model/` | Entities, schemas, migrations, ownership, lifecycle, or storage semantics are central enough to reason about independently. |
| `NN-interface/` | CLI commands, config files, APIs, UI routes, or other user/system surfaces need stable contracts. |
| `NN-verification/` | Test architecture, validation loops, conformance, benchmarks, or e2e checks need their own contract. |
| `NN-integrations/` | External systems impose assumptions, constraints, references, auth, or compatibility boundaries. |
| `NN-operations/` | Runtime operation, observability, rollout, recovery, capacity, or resource bounds need an explicit contract. |

These are subsystem VRS nodes with their own `requirements.md` and `spec.md`,
not top-level files such as `datamodel.md`.

## Existing VRS References

When creating or reshaping a VRS tree, inspect existing VRS systems as reference
examples when helpful. Match the example to the problem shape: flat topic,
hierarchical subsystem tree, composable contract/realization tree, or
package-local docs, or intuition-heavy narrative tree. Discover current
examples by searching VRS roots rather than relying on a stale inventory. Use
examples for precedent and calibration, not as templates to copy blindly.

## Review Smells

VRS smell checks are part of the meta-VRS contract. They are not a per-topic
artifact file; they are review rules agents apply when creating or editing VRS.

Common smells:

| Smell | Likely fix |
| --- | --- |
| Vision prescribes technology or architecture | Move mechanism to requirements/spec or a decision record. |
| Requirements describe implementation | Move the mechanism to spec; keep the requirement testable. |
| Spec carries rationale prose | Promote rationale to a decision record. |
| Open questions contain resolved answers | Move the answer to its owner and remove the question. |
| Roadmap constrains current implementation | Promote it to requirements/spec or mark it non-normative. |
| Delta is stale, vague, or completed | Update or delete the delta. |
| Generic `README.md` is used as VRS entry point | Use `intuition.md`. |
| Companion directories exist empty | Delete them until they contain real content. |
| One requirements file exceeds the size bound | Split into child VRS nodes. |

## Grill VRS

`grill-vrs` is the Socratic interview/update procedure that applies the VRS
contract.

VRS owns the artifacts, lifecycle, and correctness contract. `grill-vrs` owns
the conversation procedure: ask one question at a time, challenge fuzzy terms,
validate assumptions, compare against code and references, and update the
correct VRS artifact as understanding crystallizes.

The procedure must not duplicate detailed artifact rules that live in
`context/vrs/`; it should point to the meta-VRS contract and stay focused on how
to conduct the interview.

Ontology handling is part of `grill-vrs`, not a separate skill by default.
When fuzzy, conflicting, or overloaded domain language appears, `grill-vrs`
resolves the term with the user or available evidence and updates `ontology.md`
using the meta-VRS ontology contract.

## Evaluation

VRS and VRS skills are evaluated through isolated scenario runs. An eval creates
a small but non-trivial temporary VRS tree outside tracked project files, applies
the meta-VRS contract and `grill-vrs` procedure, then reports:

- whether artifact routing was clear;
- whether assumptions, constraints, decisions, ontology terms, open questions,
  references, experiments, roadmap, and deltas were distinguishable;
- whether decision evidence/options/tradeoffs were practical;
- whether the skill duplicated, contradicted, or failed to apply the meta-VRS;
- what contract gaps should become open questions, deltas, decisions, or spec
  changes.

Eval artifacts are evidence. Persist durable findings into the owning VRS
artifact; do not treat the temporary scenario tree itself as normative.

## Enforcement

VRS enforcement checks real repository VRS artifacts for hygiene and contract
violations. It is intentionally narrower than "tooling": enforcement owns check
layers, diagnostics, and gate semantics, while artifact semantics stay with the
file-kind and concept nodes that define them.

Deterministic enforcement covers mechanical invariants such as filenames,
sections, IDs, links, companion record shapes, empty companion directories,
proposed decision hygiene, and obvious stale delta markers. Semantic review
covers judgment-heavy questions such as whether requirements are testable,
whether specs are implementable, and whether decisions contain real evidence and
tradeoffs.

The enforcement contract lives in [16-enforcement](./16-enforcement/spec.md).
