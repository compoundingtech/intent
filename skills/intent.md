---
description: Author, review, and check an `intent/` corpus — the durable-intent doc contract — from any project, no fleet tooling assumed. Covers the artifact set, the `intent` CLI, and the failure modes that make a passing check prove nothing. Use when writing or reviewing intent artifacts, adopting the contract elsewhere, or wiring the checker into CI.
agents: [coding]
---

# intent — authoring and checking the contract

The contract is specified in `intent/`, and `intent/` is written in the
conventions it specifies. That makes the corpus the authority and this skill a
guide to using it — read the corpus for the rules, read this for how to work
against them and where the tooling will mislead you.

Start at `intent/vision.md`, `intent/requirements.md`, `intent/spec.md`,
`intent/ontology.md`. Each numbered directory specifies exactly one artifact
type and is itself a small intent tree; go to the one matching the artifact you
are writing. Do not reconstruct the rules from this file — it deliberately does
not restate them, so it cannot drift from them.

## The rule the whole contract turns on

**Which artifact owns this fact?** A constraint that must hold is a
requirement. The shape that satisfies it is spec. Why that shape and not
another is a decision record. The gap between spec and reality is a `.delta/`
entry. Work in progress — plans, milestones, tickets, status — is not intent at
all and belongs outside the corpus.

Two failure modes follow, and reviewers should hunt both:

- **A fact with two homes.** Restating a requirement inside a spec means they
  can disagree later, and nothing says which one is wrong.
- **A claim with no support.** Prose asserting something the corpus cannot
  substantiate reads as settled and is not. If a claim cannot be pointed at
  something, it is an open question or a decision record, not a specification.

## Hidden directories hold a large share of the corpus

`.decisions/`, `.delta/`, `.experiments/`, and `.reference/` are dot-prefixed
by convention, and **`fd`, `rg`, most editor trees, and most CI globs skip them
by default.** Between them they carry the reasoning behind most of what the
numbered sections assert, so a survey that misses them concludes the corpus is
about half the size it is and that its assertions are unexplained.

Use `rg --hidden`, `fd --hidden`, `ls -a`. When you report a file count or a
"nothing mentions X" result, say whether hidden paths were included — that
claim is wrong far more often than it is checked.

## The checker

The checker is deterministic and consumer-agnostic: `intent/16-enforcement/`
specifies the rules so that any tool can implement them, and the shipped binary
is one such tool rather than the definition. Diagnostics carry a stable
`INTENT.ENF.<rule>` code, so a finding maps back to the rule that produced it —
match on the code, never on the message text.

**Treat identifier namespaces as public contracts.** CLI requirement IDs use
`AXE.INTENT-R*`, checker rules use `INTENT.ENF.*`, and JSON
schema versions use values such as `axe.intent.check.v1`. The `axe` component
is a stable protocol/document namespace; `intent` names the layer, artifact
system, and executable command.
These identifiers changed together during the Intent-wide rename and have no
compatibility aliases, so downstream tooling must match the current values.

Run `--help` for the authoritative surface; the commands and what each answers:

| Command | Answers |
| --- | --- |
| `check` | Does the corpus violate a mechanical rule? Link targets, decision-record shape, delta shape, proposed-decision lifecycle, reference shape, experiment shape. |
| `graph` | What did the checker actually see? Emits the derived nodes and edges as JSON. |
| `review` | Semantic review — the judgements no deterministic rule catches. Drives a coding agent. |
| `review-fixtures` | Grades that semantic review against fixtures with known expected findings, i.e. checks the reviewer. |

`check` and `graph` are pure and portable — filesystem in, diagnostics out, no
network and no model.

`review` and `review-fixtures` ship in the same binary but are **not**
standalone. They drive a coding agent through an invocation contract, taking
`--backend` and `--coding-agent` (which defaults to a `coding-agent` executable
on `$PATH`). With no such executable they fail at the capabilities preflight
before examining anything — a configuration failure, not a corpus finding, and
worth recognising as such. `review` also resolves its prompt assets from the
corpus under test, so pointing it at a tree with no `16-enforcement/` fails
naming the missing asset rather than silently reviewing against something else.

Outside a fleet that supplies an agent, `check` and `graph` are the two you
will actually run. The other two are worth knowing exist for when one appears.

### Profiles

`check` takes `--profile local|strict`, defaulting to `local`.

- `local` — authoring. Transitional rule classes surface as advisory findings.
- `strict` — CI and merge gates. Blocking rules fail, and warning classes that
  have finished migrating are promoted to blocking.

A corpus that passes `local` can fail `strict`. Gate on `strict`; a clean
`local` run is not the same evidence.

## Exit 0 is not evidence the corpus was read

This is the trap worth knowing before any other. **`check` exits 0 on an empty
directory, and on a directory containing no intent artifacts at all.** No
artifacts means no rules apply, which means no violations. Worse, `--json` on a
clean real corpus and on an empty directory differ only in the `root` path:

```json
{ "schema_version": "…", "root": "…", "profile": "strict", "diagnostics": [] }
```

There is no scanned-file count in that envelope. So a green `check` is
consistent with all of: the corpus is clean; the path was wrong; the checkout
was shallow or empty; a glob excluded everything. **`graph` does not rescue
this by exit code either** — it also exits 0 on an empty directory, returning
`"nodes": [], "edges": []`.

What discriminates is the **node count**, not any exit code:

```sh
intent check ./intent --profile strict
test "$(intent graph ./intent --json | jq '.nodes | length')" -gt 0
```

Resist pinning that to a tight floor at the current count. It goes red the next
time someone legitimately adds an artifact, and a gate that cries wolf on honest
edits gets loosened until it means nothing. `> 0` is the assertion that the
checker reached the corpus at all — which is exactly what a wrong path, an empty
checkout, or an over-eager glob destroys.

`> 0` will not catch **partial** loss. Cover that with a second gate of a
different shape rather than by tightening the first: assert that specific named
artifacts you expect are present in the graph. Two gates that fail for different
reasons are worth more than one exact-match gate that fails for both — and that
you will end up disabling.

The same reasoning applies to any check whose subject might be absent: **a
passing assertion over an empty set is not a passing assertion.** Prove the set
was non-empty separately.

## Adopting the contract elsewhere

Nothing in the contract depends on the repository that defines it. The
artifacts are plain Markdown with a naming discipline; a text editor authors
them and the checker is optional at authoring time.

Adopt in this order:

1. **Copy the discipline, not the corpus.** Take the artifact set and the
   ownership rule. `intent/`'s own content is about the contract itself and is
   not a template for your domain.
2. **Start with `requirements.md` + `spec.md`.** Add `vision.md` only when the
   "why" is not obvious from context, `ontology.md` once terms are being used
   precisely enough that a reader could get one wrong.
3. **Add `.decisions/` the first time a choice is hard to reverse, surprising
   without context, and a genuine trade-off.** Fewer, real records beat a
   record per commit.
4. **Wire `check --profile strict` into CI with the non-empty graph assertion**
   above, from the start. Retrofitting enforcement onto a corpus that has
   already drifted is the expensive order.

Keep IDs stable once referenced. Renumbering is a whole-corpus edit — update
every reference in the same commit, because a stale ID reference is not a
dangling link and the checker will not catch it.
