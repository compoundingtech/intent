# Order child Intent nodes by formal contract

Status: accepted

## Context

The root Intent contract needs child nodes for each artifact kind. The order communicates
how readers should approach the system and how agents should project the skill
onto the normative contract.

## Evidence and Argument

The user explicitly rejected putting `intuition` first and preferred "Intent first,
then glossary after the spec, and then intuition after the glossary." Existing
Intent practice treats Vision, Requirements, and Spec as the formal chain, while
glossary and intuition support that chain.

## Options

| Option | Tradeoffs |
| --- | --- |
| Intuition first | Good reader onboarding, but makes the narrative entry point look like the first normative layer. |
| Lifecycle order | Useful for workflow, but obscures the Intent contract structure. |
| Formal contract first | Preserves Intent semantics: `vision` -> `requirements` -> `spec`, then language and narrative supports. |

## Decision

Order root Intent contract child nodes by the formal Intent contract first: `vision`,
`requirements`, and `spec`. `glossary` follows the spec because language is
shared by the formal documents, and `intuition` follows the glossary because it
is narrative support rather than the first normative layer. Numeric prefixes
carry reading order only; stable identifiers remain semantic.
