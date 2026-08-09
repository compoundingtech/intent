# Keep enforcement consumer-agnostic

Status: accepted

## Context

The Intent enforcement spec originally included a consumer table that named
`intent`, Nix checks, `grill-intent`, and future `axe plan` behavior. A real
`codex` semantic review of `intent/16-enforcement` flagged this as a
semantic-ownership smell: enforcement was starting to assign command-surface and
workflow behavior that should be owned by the consuming subsystems.

The user chose a strict ownership split: enforcement should define checks,
severity, evidence, semantic review assets, and reusable interfaces only; Axe
Intent and other consumers should own their command surfaces and provider
invocation details.

## Evidence and Argument

The ownership issue was found by a bounded manual real-provider run of
`intent review intent/16-enforcement --backend codex` after the initial
ownership cleanup. The review succeeded but still reported
`INTENT.REVIEW.semantic-ownership` against the enforcement spec's consumer table.

The existing Intent CLI spec already owns CLI behavior, provider-readiness checks,
stdout/report routing, and CI token-spend guards. Keeping consumer behavior in
the enforcement spec duplicates that contract and creates drift risk. A
consumer-agnostic enforcement contract also composes better with future
consumers because they can reuse diagnostics and rule metadata without inheriting
Axe-specific workflow semantics.

## Options

| Option | Tradeoffs |
| --- | --- |
| Strict ownership split | Cleanest authority boundary and directly addresses the semantic-review finding; requires readers to follow links to consumer specs for concrete command behavior. |
| Shared summary table | More readable in one place, but it keeps inviting duplicated command semantics and future ownership drift. |
| Move the consumer map to intuition | Preserves orientation as non-normative prose, but still risks stale guidance unless carefully maintained. |

## Decision

Intent enforcement is consumer-agnostic.

The enforcement spec defines:

- deterministic and semantic enforcement layers;
- rule maturity and gate policy;
- diagnostic shape;
- baked semantic-review prompt and result schema;
- reusable checker/review interfaces.

Consumer specs define their own command surfaces, invocation policies, storage,
report routing, and exit-code behavior. In particular, the Intent CLI spec owns
`intent` commands and provider execution policy.

## Consequences

- Enforcement no longer enumerates concrete consumer command behavior.
- Semantic-review findings about Intent ownership should route command-surface
  details to the relevant consumer Intent instead of `intent/16-enforcement`.
- Future consumers can reuse enforcement outputs without becoming part of the
  enforcement contract.
