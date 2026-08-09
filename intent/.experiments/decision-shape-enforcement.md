# Decision Shape Enforcement

## Question

Can decision-record shape checks become a useful deterministic Intent enforcement
rule without pretending to judge semantic decision quality?

## Hypothesis

Decision-record shape checks can become a useful deterministic Intent enforcement
rule if they catch missing evidence, options, tradeoffs, and rationale without
pretending to judge semantic decision quality.

## Method

Three evidence paths were explored:

- fixture/prototype direction: design good and bad decision records and test
  whether a mechanical checker distinguishes missing structure from compliant
  records;
- corpus dry run: scan existing decision records for the root Intent contract minimum
  shape;
- agent-eval direction: compare decision drafting with only prose guidance
  versus drafting with an explicit checker-style rubric.

A local corpus scan also checked all current `context/**/.decisions/000*.md`
records with a simple mechanical predicate:

- `Status:` present;
- `## Context`, `## Evidence and Argument`, `## Options`, and `## Decision`
  present;
- options section uses an `Option | Tradeoffs` table.

## Result

The root Intent contract corpus is compatible with the proposed mechanical minimum: the
current `intent/.decisions/000*.md` records use the required sections and
options-table shape.

The wider repository corpus is not ready for repo-wide blocking promotion. A
local scan found 51 durable decision records under `context/**/.decisions/` and
40 records missing at least one part of the proposed strict mechanical shape.
Most misses are older or subsystem-local records that predate the current
root Intent contract decision contract.

The agent-eval direction found that checker-style guidance improves authoring
behavior: agents are less likely to skip evidence, options, tradeoffs, and
selected-option rationale. It also showed the core limitation: an agent can
satisfy headings with weak or decorative content, so a deterministic checker
cannot prove evidence quality or principled alternatives.

## Promising Mechanical Subset

These checks look safe as deterministic warnings after migration planning:

- status line exists and uses an accepted status;
- required headings exist: `Context`, `Evidence and Argument`, `Options`,
  `Decision`;
- required sections are non-empty;
- options table contains at least two option rows;
- option rows contain non-empty tradeoff text.

## Unsafe As Blocking Deterministic Claims

These remain semantic review concerns:

- whether evidence is real or strong enough;
- whether options are genuinely principled;
- whether tradeoffs capture the actual constraint tension;
- whether the selected option is truly best.

## Conclusion

Decision-shape enforcement is promising, but not ready as a repo-wide blocking
gate. It should remain warning/fixture mode until:

- fixtures cover good records, missing-section records, malformed options, and
  section-stuffed records;
- the current repo corpus is migrated or the rule scope is explicitly limited;
- semantic review remains responsible for substance quality.

Local Markdown link existence should remain the first blocking deterministic
gate. Decision shape is a second candidate for the mechanical subset only after
fixture and corpus evidence are green.

## Intent Impact

Supports the deterministic enforcement scope in
[16-enforcement/spec.md](../16-enforcement/spec.md): decision-shape checks are
valuable for the root Intent contract corpus, but broad blocking promotion should wait for
fixture coverage and migration of older decision records.
