# Start xref enforcement with a deterministic subset

Status: accepted

## Context

After deciding that Intent identifiers are commit-scoped consistency handles, the
user agreed that xref tooling should enforce this rule. The link-structure
experiments showed that Markdown links are useful for navigation, wiki-style
links are ambiguous in hierarchical Intent, and symbolic or namespaced IDs make
refactors easier when a resolver can check them.

## Evidence and Argument

The current Intent representation is still Markdown-authored. A full typed graph is
on the roadmap, not current infrastructure. However, a narrow deterministic xref
subset can provide immediate value without requiring semantic review or a full
registry: link existence, ID uniqueness within scope, spec-to-requirement
resolution, cross-node namespace/link requirements, and warning on normative
wiki links.

This matches the commit-scoped ID model: the checker validates that one commit
is internally consistent, not that IDs stay stable forever.

## Options

| Option | Tradeoffs |
| --- | --- |
| Wait for typed Intent graph | Cleaner architecture later, but leaves current brittle links unchecked. |
| Enforce all reference semantics now | Ambitious, but likely noisy without a resolver and migration plan. |
| Start with deterministic xref subset | Immediate value, low ambiguity, and compatible with future typed graph work. |

## Decision

Start xref enforcement with a narrow deterministic subset. Make mechanically
resolvable link and ID consistency checks blocking after calibration; initially
warn on wiki-style links in normative artifacts and promote them to errors only
after migration is understood.
