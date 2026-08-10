# Intent Enforcement — Spec

This document specifies Intent enforcement for real repository Intent trees. It builds
on [requirements.md](./requirements.md).

## Status

Draft.

## Scope

Enforcement checks whether Intent artifacts obey the root Intent contract. It does not
define artifact semantics; it consumes the contracts owned by sibling nodes such
as [02-requirements](../02-requirements/spec.md),
[03-spec](../03-spec/spec.md), [06-decisions](../06-decisions/spec.md),
[09-delta](../09-delta/spec.md), and
[13-review-smells](../13-review-smells/spec.md).

Enforcement is distinct from [15-evaluation](../15-evaluation/spec.md):

| Concept | Target | Output |
| --- | --- | --- |
| Evaluation | Temporary scenario outside tracked files | Evidence about whether Intent/skills work end to end |
| Enforcement | Real repository Intent artifacts | Findings about hygiene, validity, and review risk |

## Enforcement Layers

| Layer | Checks | Default Gate |
| --- | --- | --- |
| Deterministic lint | File names, required sections, IDs, links, companion shapes, empty directories, proposed records, obvious stale delta markers | Merge-blocking once implemented and calibrated |
| Semantic review | Vision purity, requirement testability, spec completeness, decision substance, open-question freshness, review-smell judgment | Advisory until an Intent or project policy makes it blocking |
| Eval feedback | Reproduces false positives, missing checks, and ambiguous diagnostics in isolated scenarios | Evidence for Intent updates |

## Deterministic Check Candidates

The first deterministic enforcement surface should prioritize checks that are
cheap, local, and low-ambiguity:

- Intent node detection and allowed lazy companion directories.
- Numeric child directory names (`NN-slug`) where child Intent nodes are used.
- Required sections for each artifact kind.
- `spec.md` status and link to its `requirements.md`, with explicit exceptions
  for root nodes if needed.
- ID syntax, uniqueness, and gap-free numbering within the declared scope.
- Requirement count warning above 30 and failure at the hard limit of 40.
- Decision filename, status, and required sections.
- Non-empty `.decisions/.proposed/` before merge.
- Companion record shapes for `.delta/`, `.experiments/`, and `.reference/`.
- Relative Markdown links and ID references where mechanically resolvable in the
  current commit.

## Cross-Reference Enforcement

Xref enforcement applies the commit-scoped identifier contract from
[02-requirements](../02-requirements/spec.md) and
[03-spec](../03-spec/spec.md). It starts with a narrow deterministic subset:

| Check | Gate |
| --- | --- |
| Relative Markdown links resolve to existing files or anchors when anchors are mechanically derivable. | Blocking after link cleanup and allowlist calibration |
| ID definitions use the accepted syntax and are unique within their declared scope. | Warning first; blocking after legacy syntax and duplicate-scope cleanup |
| Specs reference requirements that resolve unambiguously in the current commit. | Warning first; blocking after local scope rules are calibrated |
| Cross-node references are namespaced or include a Markdown link to the owning artifact. | Warning first; blocking after migration |
| Wiki-style links in normative artifacts point to docs, decisions, requirements, or specs. | Warning first; error after migration |

This is not a full typed Intent graph. The first checker should avoid semantic
judgment and should report ambiguous references instead of guessing. If a rule
cannot resolve scope from the current artifact and links, it emits a diagnostic
that names the missing scope declaration or owning artifact.

Warning mode is a transitional migration state, not a permanent operating mode.
Every deterministic warning class must carry an exit condition: the known corpus
is migrated, explicit allowlists are reviewed, false positives are encoded as
fixture/evaluation cases, and the remaining rule is either promoted to blocking
or deleted. Transitional checks should not accumulate special cases once the
migration has completed; remove migration-only complexity when strict mode is
enabled.

Initial migration ordering:

1. Fix or explicitly allowlist known broken local Markdown links, then make file
   link existence blocking.
2. Experiment with decision-record shape checks for required status, context,
   evidence, options, tradeoffs, and decision sections. Keep them warning-only
   until fixtures show the rule catches real missing substance without
   encouraging decorative section stubs and the target corpus is migrated or
   explicitly scoped; then promote the mechanically checkable subset to
   blocking.
3. Warn on ID definition syntax and duplicate IDs while legacy syntax and
   duplicate-scope cases are migrated; then make accepted-syntax and declared
   scope uniqueness blocking.
4. Warn on ambiguous bare IDs, cross-node references without an owning link, and
   normative wiki-style doc links until the corpus is migrated; then make the
   mechanically resolvable subset blocking.
5. Keep ontology-style term shorthand separate from normative doc references so
   term-link convenience does not block the stricter artifact-reference rule.

The first blocking rule should be local Markdown link existence because it is
cheap, local, low-ambiguity, and directly improves navigation. Decision-record
shape is the second blocking candidate only after experiments are promising. The
initial decision-shape experiment found the root Intent contract decisions compatible with
the proposed strict shape, but the wider `context/**/.decisions` corpus is not
ready for repo-wide blocking without migration or scoped enforcement.

Initial strict decision-shape enforcement applies only to
`context/intent/.decisions/`. Other `context/**/.decisions/` records may be scanned
for migration diagnostics later, but they are not part of the first strict
decision-shape scope.

## Semantic Review Candidates

Semantic enforcement should classify findings with evidence instead of pretending
judgment is deterministic:

- `vision.md` prescribes architecture, implementation, or tooling.
- `requirements.md` contains implementation mechanisms instead of testable
  constraints.
- `spec.md` is not detailed enough to implement or does not trace to
  requirements.
- Decision records contain decorative evidence/options/tradeoffs rather than
  substantive reasoning.
- `open-questions.md` contains resolved questions or questions without real
  blockers.
- Review-smell findings require a human or agent to decide whether the smell is
  real in context.

The baked semantic-review prompt and output schema live in this enforcement
node:

- [review-prompt.md](./review-prompt.md) defines the default `intent review`
  prompt and routes semantic judgment to the owning Intent contracts.
- [review-result.schema.json](./review-result.schema.json) defines the
  `axe.intent.review.v1` result shape.

`intent review` consumes these assets through the Coding Agent Invocation
Contract. Callers do not provide arbitrary prompts for the standard review
mode; prompt and schema changes are Intent changes and should be evaluated through
fixtures before becoming the baked version.

Real-provider execution policy is owned by the
[Intent command spec](../../cli/spec.md). Enforcement
owns the baked prompt, result schema, semantic finding shape, and fixture-backed
quality evidence. The command spec owns when providers may run, which CAIC
contract pieces are required, and how stdout/report routing behaves.

## Diagnostic Shape

Structured enforcement output should preserve enough routing context for agents:

```json
{
  "kind": "deterministic | semantic",
  "severity": "error | warning | info",
  "gate": "blocking | advisory | review",
  "artifact": "context/intent/requirements.md",
  "owner": "02-requirements",
  "rule": "INTENT.ENF.<rule-id>",
  "evidence": "Requirement IDs skip INTENT-R12.",
  "suggested_fix": "Renumber the requirement or update references."
}
```

## Checker Interface

Deterministic enforcement should be implemented behind a checker interface that
can be consumed by multiple operator surfaces without duplicating rule logic.
The checker owns mechanical parsing, rule evaluation, derived graph extraction,
and diagnostic emission. It does not own the semantics of the artifacts it
checks; those stay in the relevant root Intent contract nodes.

Enforcement owns the reusable checking contract, not command-surface behavior.
Consumers may call this interface from CLIs, Nix checks, review workflows, or
future planning tools, but each consumer owns its own operator UX, invocation
policy, storage, and exit-code integration.

The enforcement interface should expose:

- rule registry and rule maturity (`warning`, `blocking`, `advisory`, or
  retired);
- diagnostic JSON using the shape above;
- rule-profile selection (`local`, `migration`, `strict`);
- derived graph JSON for mechanically resolved nodes and edges;
- exit-code semantics for strict profiles.

## Workflow Integration

Deterministic enforcement should integrate with the repo's local check workflow
before becoming a CI expectation. Semantic review should be available as an
explicit agent or LLM-backed review mode that reports findings by owner artifact
and does not silently rewrite Intent.

`grill-intent` may call enforcement during an Intent session, but it remains the
interview/update procedure. Enforcement supplies findings; the owning Intent
artifact still receives the durable fix.

## Gate Policy

Deterministic checks are the default merge-blocking path once their migration
exit conditions are met. A deterministic check starts as warning when the corpus
still needs cleanup, fixture coverage, or false-positive calibration; after that
migration completes, the rule is either promoted to blocking or deleted.

Semantic review is advisory by default. It may become blocking only for a
specific Intent or project policy after evidence shows that the review mode is
reliable enough for that scope and has a clear appeal or override path. Until
then, semantic findings route to review comments, deltas, decisions, or open
questions owned by the affected Intent artifact.
