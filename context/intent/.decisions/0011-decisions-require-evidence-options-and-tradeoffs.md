# Decisions require evidence, options, and tradeoffs

Status: accepted

## Context

Earlier Intent decision records used a minimal "what and why" shape. That is useful
for low-friction capture, but it can let weakly justified choices harden into
durable contract without showing why alternatives lost.

## Evidence and Argument

The user explicitly wants each decision to show evidence for the best approach,
principled options, proofs or arguments, tradeoffs, and the reason the selected
option is best. This also matches the Intent goal of helping future readers avoid
re-litigating hidden reasoning.

The user also clarified that evidence often should be plural: at least one
applicable evidence form is required, and important decisions should use all
feasible evidence forms that apply, such as research plus benchmarks,
experiments plus user confirmation, or proofs plus implementation evidence.

## Options

| Option | Tradeoffs |
| --- | --- |
| Minimal decision records | Fast to write, but often hides alternatives and weak evidence. |
| Full ADR-style template | Thorough, but can become ceremony and architecture-specific. |
| Evidence/options/tradeoffs minimum | Requires real reasoning while staying broad and compact. |

## Decision

Intent durable decision records must include evidence or argument, principled
options with tradeoffs, and a clear reason the chosen option is best under the
current Intent context. They must include at least one applicable evidence form and
should include multiple independent evidence forms when the decision is
important, risky, or expensive to reverse. This keeps the record broad enough
for non-architecture decisions while preventing unsupported choices from
becoming durable truth.

## Amendment 1: a claim's scope may not exceed its evidence's scope (evidence-driven)

The requirements above govern the *presence and plurality of evidence forms*.
They do not govern the relationship between what a claim asserts and what its
evidence actually covers. A record can fully satisfy this decision — run a scan,
cite it — and still state "repo-wide" over a one-directory scan, or "every" over
a `head`-limited one.

**Rule.** A durable claim must not assert a scope wider than the scan behind it
demonstrates. When a claim reaches for a summarizing word — `every`, `all`,
`no`, `none`, `most`, `repo-wide` — that word is a signal to re-run the
underlying check unbounded before the claim lands, not a flourish. A count or
scope figure must carry the counting convention inline, because a bare number
inherits whatever convention the reader brings (e.g. "220 shadowing
relationships", not "220 ordinals" for a figure that is 39 ordinals under the
reading the word "ordinals" invites).

**Evidence.** Six independent instances surfaced across two agents in one
working session (2026-07-21): an attribution inferred from `git log -1 -- <file>`
(which reports last-touch, not creation); "220 ordinals" for a 220-relationship
/ 39-ordinal count; a "most-collided" superlative asserting an uncomputed
ordering (actual max 3.5x larger); "every bare cite resolves, none
cross-subsystem" from a `head -15` over 17 hits; "a repo-wide sweep found only
these three" scanned over one subtree; and "X is cited in artifact Y" where Y
said no such thing. Mechanism identified and two instances independently
re-derived, meeting this decision's own evidence bar. Prior art:
observability's `0024` documented an analogous collision class and deferred the
fix to "a separate housekeeping pass" that never ran — a deferral that outlived
its reasoning.

**Two causes of under-correction, because the fix differs by cause.**

1. *Data uniformity.* A truncated scan misleads least visibly exactly when the
   unexamined remainder agrees with the examined part — which is also the
   condition that made truncating feel sufficient. The failure is selectively
   unsurfaced precisely where re-deriving feels least necessary.
2. *Incentive.* An error that benefits the party best positioned to catch it is
   structurally under-corrected, because the only cheap checker has no reason to
   look. This is **not closable by the author being more careful** — the author
   is who lacks the reason to check. Any agent reporting its own work to a
   coordinator is in this configuration by default.

**Consequence for practice.** Because cause (2) is not closable by author
vigilance, the operative rule is not "re-derive your own claims" but "have the
claim re-derived by someone who does not benefit from it." Cross-checking
between agents does work individual care structurally cannot. This applies to
attribution claims (who said X) as much as to factual ones: they are checkable
in the record and are not a softer category.

**Limit.** This is a discipline, not a gate. The companion enforcement in
`intent/.decisions/0033` constrains *citations* (an ordinal must resolve;
a label must match its target); it cannot see a *claim* — prose asserting a
fact about the tree passes every structural check while being false. The
enforcement narrows what needs re-derivation; it does not remove the
obligation.
