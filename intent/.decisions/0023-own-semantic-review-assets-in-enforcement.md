# Own semantic review assets in enforcement

Status: accepted

## Context

`axe vrs review` needs a baked prompt and output schema so semantic review can
run through the Coding Agent Invocation Contract without accepting arbitrary
caller prompts.

The ownership question is whether those assets belong with the enforcement
contract, the review-smell rubric, the Axe command VRS, or packaged Axe runtime
assets.

## Evidence and Argument

The CAIC semantic-review prototype showed that `axe vrs review` can pass a
baked prompt, generated deterministic diagnostics, normative VRS artifacts, and
an output schema through `$CODING_AGENT` in read-only review mode. It also
showed that generated diagnostics need stable schema markers before review is
production-ready.

Semantic review is an enforcement mode: it classifies VRS hygiene and review
risk, emits findings with gates, and composes deterministic diagnostics with
human or agent judgment. The review-smell catalog supplies the rubric, but does
not own invocation policy or output schema. Axe owns command plumbing and
packaging, but making Axe own the prompt would make a tool implementation the
source of VRS semantics.

## Options

| Option | Tradeoffs |
| --- | --- |
| `context/vrs/16-enforcement/` | Keeps prompt and schema with the enforcement contract and makes VRS the semantic owner, but puts executable review assets in a spec node. |
| `context/vrs/13-review-smells/` | Keeps the prompt close to the smell rubric, but separates review output schema and gate policy from enforcement. |
| `context/coding-agents/14-axe/12-vrs/` | Keeps assets close to the command implementation, but risks Axe becoming the semantic authority for VRS review. |
| `flakes/axe/assets/vrs-review/` | Easiest to package in the CLI, but separates normative review behavior from the VRS source of truth. |

## Decision

The baked semantic-review prompt and output schema are owned by
`context/vrs/16-enforcement/`.

The first assets are:

- `context/vrs/16-enforcement/review-prompt.md`;
- `context/vrs/16-enforcement/review-result.schema.json`.

`axe vrs review` consumes those assets. It does not define an alternative prompt
contract and does not accept arbitrary prompts for the standard review mode.

## Consequences

- Prompt and schema changes are VRS changes, not Axe-only implementation
  changes.
- The review-smell catalog remains the rubric source, but enforcement owns the
  runnable review contract.
- Axe can package or reference the assets, but packaging must preserve VRS
  ownership.
- Eval fixtures should tune the prompt and schema before semantic review becomes
  a production gate.
