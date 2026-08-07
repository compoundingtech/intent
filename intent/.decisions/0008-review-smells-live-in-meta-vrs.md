# Review smells live in meta-VRS

Status: accepted

## Context

The Notion notes called out VRS smells and bad patterns. The user chose to put
those checks in the meta-VRS rather than a separate per-topic artifact or only
in the skill.

## Evidence and Argument

Smells are review rules for VRS authors and agents. They are not durable facts
about each documented system, so per-topic `smells.md` files would drift and
create noise. Keeping smells in the meta-VRS lets skills apply the same review
contract uniformly.

## Options

| Option | Tradeoffs |
| --- | --- |
| Per-topic `smells.md` | Local and visible, but likely noisy and stale. |
| Skill-only guidance | Easy to apply operationally, but not normative outside the skill. |
| Meta-VRS review-smells node | Central, normative, and reusable by skills without becoming a topic artifact. |

## Decision

VRS smells and bad patterns live in the meta-VRS contract. They guide review of
all VRS documents but are not per-topic artifacts. This centralizes checks such
as stale open-question pruning, wrong-artifact ownership, and generic README
usage.
