# Rename the glossary artifact to ontology

Status: accepted

## Context

The Intent language artifact was `glossary.md`: a flat list of canonical terms and
definitions (a lookup). In practice several nodes already carried relationship
structure inside it (anchors, siblings, composition seams), which a glossary
does not model. Separately, the reusable discipline of *designing* a term
system — relationships, leitwort hierarchy, exposure/priority — had no home;
[0015](./0015-glossary-handling-lives-in-grill-intent.md) deferred a separate
skill.

## Evidence and Argument

Existing language artifacts already combine canonical definitions with
relationships such as anchors, siblings, and composition seams. Calling that
combined job a glossary describes only its lookup layer, while an ontology
names both the minimal language layer and the optional structure those
artifacts already need.

One `ontology.md` keeps the former glossary as its minimal form and adds
structure only when relationships carry weight. That preserves a single
artifact boundary without forcing structure into every subsystem. Keeping
separate glossary and ontology files would split one job across two artifacts,
while purging the historical term would rewrite records and fixtures whose
wording remains true.

The separate `/sk-ontology` and `/sk-naming` disciplines also satisfy the
unowned term-design concern recorded by
[0015](./0015-glossary-handling-lives-in-grill-intent.md) without moving the Intent
interview's responsibility for applying the artifact contract.

## Options

| Option | Trade-off |
| --- | --- |
| Keep `glossary.md`; add a separate `ontology.md` | Two artifacts for one job; unclear seam between lookup and structure. |
| Full concept purge of "glossary" | Consistent, but rewrites immutable history and eval fixtures. |
| Rename the artifact `glossary.md` → `ontology.md` | One artifact whose minimal form is the old glossary; structure is a lazy add. |

## Decision

The Intent language artifact is `ontology.md`. An ontology is the language layer
(canonical terms — the former glossary, still the minimal form) plus an
optional structure layer (relationships + leitwort), added lazily. The
term-design discipline lives in two general skills — `/sk-ontology` (the term
system) and `/sk-naming` (a single term) — which satisfy the condition
[0015](./0015-glossary-handling-lives-in-grill-intent.md) reserved for a separate
skill. This supersedes 0015: `grill-intent` still owns applying the ontology
contract during interviews (INTENT-R24, INTENT.GRILL-R08); the skills own the
reusable discipline it applies. The subsystem contract moves to
[../04-ontology/](../04-ontology/).

Existing `glossary.md` files across `context/` migrate to `ontology.md`
mechanically; historical decision records keep their original "glossary"
wording as history.
