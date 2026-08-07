# SOTA survey — ontology / terminology / naming engineering

## Question

Which established knowledge-organization practices fit a lightweight,
text-first engineering ontology, and which would add academic weight without
improving the contract?

## Method

This non-normative evidence trail surveys established external standards (SKOS,
OWL, faceted classification, DDD, schema.org), maps them to our concepts, and
adversarially reviews the fit.

## Result

The adopted result is one edge vocabulary
(`isa`/`partOf`/`refines`/`dependsOn`/`related` + `altLabel`/`avoid`) shared
across `/sk-ontology`, the `04-ontology` contract, and `#1180`; keep the
is-a/part-of split (more precise than SKOS); cite-don't-teach the standards.
Goal below: adopt what a text-first engineering context benefits from; reject
academic weight; lock ONE edge vocabulary so the artifacts don't drift.

Bottom line up front:
- Our concepts are already well-aligned to SOTA. The biggest wins are *naming
  the lineage* (SKOS/faceted/DDD) and *unifying one edge vocabulary*, not adding
  machinery.
- We are, correctly, more precise than SKOS on one axis (we split is-a vs
  part-of; SKOS conflates them) and we should keep that.
- Almost none of the SOTA belongs *inside* the skills as prose — Claude already
  knows SKOS/OWL/DDD. It belongs as one-line pointers plus a shared vocabulary.

---

## 1. SOTA landscape (and what maps to us)

### The KOS ladder — positions our `ontology.md` artifact
Knowledge Organization Systems form a complexity ladder, each level a superset of
the last:

| Level | Adds | Our equivalent |
| --- | --- | --- |
| **Controlled vocabulary** | one authorized term per concept | `## Language` list + `VRS.ONT-R01` canonical term |
| **Taxonomy** | hierarchy (broader/narrower) | `## Structure` with hierarchy |
| **Thesaurus** | associative (`related`) + synonyms (`altLabel`) | `related` edges + `_Avoid_` / alt labels |
| **Ontology** | typed, specialized relations + formal semantics | #1180 typed edges (`isa`/`partOf`/`dependsOn`/…) |

This ladder is exactly our "lazy structure" rule (`VRS.ONT-R06`): a flat glossary
is a controlled vocabulary; you climb only when a relationship carries weight.
**Adopt this as the one-line backbone** for the artifact's levels — it gives
"minimal ontology = flat term list" a recognized name (controlled vocabulary) and
frames when to add each layer. (Sources: taxonomies-sig, Accidental Taxonomist.)

### SKOS — the pragmatic, text-first sweet spot; closest match to us
SKOS (W3C Rec) is the intentionally-lightweight middle of the road: an RDF
vocabulary for thesauri/taxonomies without OWL's logical commitments. It is the
single closest external model to what we want. Two tiers (SKOS itself separates
them):

- **Lexical labels** (point at *strings*): `prefLabel` (one per language =
  canonical term), `altLabel` (synonyms), `hiddenLabel` (findable, not
  displayed).
- **Semantic relations** (point at *concepts*): `broader`/`narrower`
  (hierarchy), `related` (associative). Cross-scheme variants: `broadMatch`,
  `narrowMatch`, `relatedMatch`, `closeMatch`, `exactMatch`.

Key mappings and one deliberate divergence:
- `prefLabel` == our canonical term (`VRS.ONT-R01`).
- `altLabel` == our true synonyms; `avoid` ≈ `hiddenLabel` **(loose** —
  hiddenLabel means "match on it but don't show it"; ours means "don't use it").
- `related` == our "orthogonal / see-also" associative link.
- **Divergence (in our favor):** SKOS `broader`/`narrower` *deliberately
  conflates* subclass and part-of — the spec explicitly notes it covers both
  "genre/species" and "whole/parts." We split **hierarchy vs composition**, which
  matches OWL/BFO and is more precise. Keep the split; don't collapse to SKOS.
- **Useful integrity rule:** SKOS makes `related` **disjoint** from
  `broader`/`narrower` (S27) — a concept pair can't be both hierarchical and
  merely associative. This is a free resolver check for #1180.
(Sources: W3C SKOS Reference & Primer.)

### OWL / RDF + ontology design patterns — mostly too heavy, two nuggets
OWL is where formal semantics live (`subClassOf`, transitivity, `sameAs`,
reasoners). We do **not** want OWL's weight. Two things transfer:
- **part-of is a first-class, transitive relation** (OWL has no built-in
  primitive; the standard pattern defines a transitive `partOf`). Backs our
  separate `composition`/`partOf` edge and implies a resolver could treat
  `partOf`/`isa` as transitive for closure checks.
- **`owl:sameAs` is "powerful and dangerous"** — merging two identifiers that
  aren't truly identical produces nonsense. Cautionary note for any future
  `exactMatch`/alias-merge between subsystem ontologies: prefer `altLabel` within
  a scheme over cross-scheme identity assertions.
ODPs (logical/architectural/content/presentation) are academic overkill here.
(Sources: W3C Simple Part-Whole Relations; OWL Guide.)

### Faceted classification (Ranganathan) — the real name for our "orthogonal"
Ranganathan's facet analysis (PMEST; Colon Classification, 1933) organizes a
subject as **independent axes combined freely** rather than one rigid tree — the
exact meaning of our **orthogonal (⟂)** relationship. Modern lineage: faceted
search/filters, and **schema.org's multiple-domain/range** decision (a property
like `startDate` is reused across unrelated types instead of forcing a common
supertype). "Facet" is the established one-word name for "independent dimension";
worth borrowing over "orthogonal axis." (Sources: IAI facet-analysis, Britannica,
schema.org CACM.)

### schema.org — our pragmatism role model
schema.org explicitly "trades elegance and global consistency for pragmatic
tolerance… unlikely to take on reorganizations motivated by ontological purity."
Polyhierarchy + multiple domains/ranges + "choose not to add more detail." This
is the disposition our skills should cite as the north star for an *engineering*
(not academic) ontology: derive value, refuse formalism-for-its-own-sake.
(Source: schema.org, CACM.)

### BFO / upper ontologies — do NOT adopt; one incidental validation
BFO (ISO/IEC 21838-2) splits **continuant** (things that persist) vs
**occurrent** (things that unfold — processes/events). Right for biomedical data
integration; massive overkill for a glossary. The only transferable crumb: the
continuant/occurrent split loosely validates `/sk-naming`'s **noun-vs-verb**
decision (a thing → noun; an action → verb). Cite as at most one clause; do not
introduce upper-ontology vocabulary. (Sources: BFO/Wikipedia, Barry Smith.)

### DDD ubiquitous language / bounded contexts — the *why* of a per-subsystem ontology
Evans: one shared, consistent vocabulary **within an explicitly bounded
context**; outside it, the same word may mean something else. This is precisely
our per-subsystem `ontology.md` + the VRS `## Flagged ambiguities` mechanism for
a child that narrows a parent term. It's the strongest existing-practice
justification for *scoping* ontologies to subsystems rather than one global
glossary. Maps to SKOS **concept scheme** boundaries (below). (Sources: Evans;
Fowler, BoundedContext.)

### Leitwort — a genuine, older term (attribution fix)
Leitwort / Leitwortstil ("leading-word style") is from **literary theory**
(thematic repetition of a key word); the musical cousin is leitmotif. It was
**applied to skills/engineering by Matt Pocock**, who did not coin it. Current
skill says "Term via Matt Pocock" — soften to "from literary theory; applied to
skills/engineering by Matt Pocock." Leitwort is a *naming/grouping convention*,
not a semantic edge (it realizes hierarchy/composition in the identifier). No SOTA
KOS has a direct equivalent — it's our legitimate house contribution. (Source:
Pocock; Leitmotif/Wikipedia.)

### Naming-convention theory — affix/morphology systems
Established practice (MS Framework Design Guidelines, Google AIP-190, SAP/AL
prefix-suffix rules) backs **morphological consistency** and **affix systems** —
shared stems/prefixes signaling family and kind. This is the mainstream
engineering echo of leitwort. `/sk-naming` already covers register/verb-noun/alias
hygiene; these guidelines add nothing Claude doesn't know — **cite-don't-teach**.

---

## 2. Vocabulary reconciliation

**Framing (critical):** the skill's five relationships are *naming-shape lenses*
("what identifier shape does this imply?"). #1180's edges are *checkable SoT
relations*. They are **not competitors** — the naming-relevant edges are a
**subset** of the full typed-edge set. So we want ONE set of edge *names* shared
across skill + VRS + #1180, with a marked subset that shapes names. This closes a
**bidirectional gap**: the skill's `orthogonal` has no #1180 edge, and #1180's
`dependsOn`/`related` have no lens in the skill's five.

Two tiers, mirroring SKOS (labels vs semantic relations):

### Tier A — lexical labels (point at strings)
| Our term | SKOS | #1180 | Notes |
| --- | --- | --- | --- |
| canonical term | `prefLabel` | `label` / `prefLabel` | one per concept (`VRS.ONT-R01`) |
| synonym / alt | `altLabel` | `altLabel` | true interchangeable names |
| `_Avoid_` | `hiddenLabel` *(loose)* | `avoid` | ours = "don't use"; hiddenLabel = "match, don't show" |

### Tier B — semantic relations (point at concepts)
| Our relationship (skill's five) | SKOS | OWL | #1180 edge | Shapes name? |
| --- | --- | --- | --- | --- |
| **hierarchy** (is-a, parent/child) | `broader`/`narrower` | `rdfs:subClassOf` | `isa` | **Yes** — child carries parent leitwort |
| **composition** (has-a, part-of) | *(folded into broader)* | transitive `partOf` | `partOf` | **Yes** — part named within whole |
| **set** — subset half (⊆) | `broader` | `subClassOf` | `isa` | Yes → **fold into `isa`** |
| **set** — membership half (∈) | `skos:member` / `inScheme` | instance-of | *(scheme member)* | No → **name it "membership," not "set"** |
| **orthogonal (⟂)** = **facet** | `related` (weak) | multi-domain (schema.org) | *(gap → use `related` + "no shared leitwort")* | Yes (negatively) — names must NOT imply coupling |
| **groups / layers** | concept scheme / `Collection` | — | *(scheme + leitwort prefix)* | Yes — shared layer prefix |
| *(no lens)* — directed dependency | — | object property | `dependsOn` | No |
| *(no lens)* — associative "see also" | `related` | object property | `related` | No |
| *(no lens)* — spec specialization | `narrower` (weak) | `subPropertyOf` | `refines` | Maybe — a narrowing, ≈ `isa` flavor |

### Recommended ONE edge vocabulary (standardize across skill + VRS + #1180)
Adopt #1180's typed-edge nouns as the canonical set; they already mirror
SKOS/OWL and are the machine-checkable SoT everything else projects from:

```
isa        # hierarchy / subclass / narrower   (naming-relevant)
partOf     # composition / part-whole, transitive (naming-relevant)
refines    # spec/requirement specialization    (naming-relevant, ≈ isa flavor)
dependsOn  # directed dependency                 (semantic only)
related    # symmetric associative / see-also    (semantic only)
altLabel   # synonym  (label, not concept edge)
avoid      # discouraged (label → string)
```

Reclassify the skill's five, don't delete their guidance:
- `hierarchy` → **`isa`**; `composition` → **`partOf`**.
- `set` → **drop the word**: subset ⇒ `isa`; membership ⇒ name it "membership"
  (SKOS scheme/collection). "Set" today conflates two genuinely different
  relations.
- `orthogonal` → keep as a *design lens* under the name **facet**; it is **meta,
  not an edge** (it's the *absence* of subsumption). If a link is needed, it's
  `related` + the explicit rule "no shared leitwort."
- `layers` → not a semantic edge; a **scheme/grouping** realized by a shared
  leitwort prefix (maps to SKOS concept scheme).

Mark the **naming-relevant subset** = {`isa`, `partOf`, `refines`, facet(neg),
layers}. Those are what `/sk-ontology` reasons about to shape identifiers;
`dependsOn`/`related` exist in the SoT but don't drive names.

**Resolver integrity rules for #1180** (free, from SOTA):
- `isa`/`partOf` disjoint from `related` (SKOS S27).
- `avoid` targets a string, not a concept — never resolve it as an edge.
- `isa` and `partOf` transitive → cycle detection + closure checks.
- Cross-scheme identity: prefer in-scheme `altLabel` over `exactMatch`/`sameAs`
  merges (`owl:sameAs` hazard).

**VRS inheritance backing:** #1180's cross-file import ("inherited from
`../glossary.md`") == SKOS **`inScheme` + concept-scheme boundaries**. That's the
SOTA backing for the VRS spec's "inheritance flows downward; a child defines only
new terms/structure/ambiguity." Worth one line in the contract.

---

## 3. Principled options (adopt-now vs defer-to-#1180)

Token-bar reminder: the bar applies to **skill edits**, not this report. The
report may explain SKOS fully; skill additions must be house-specific and cheap.

### The one fork that matters — how to unify the edge vocabulary
| Option | What | Trade-off | Token bar |
| --- | --- | --- | --- |
| **3A — rename the five to edge-names now** | `/sk-ontology` + VRS `04-ontology` adopt `isa/partOf/refines/dependsOn/related` (+ `altLabel`/`avoid`), mark naming-relevant subset, drop `set`, rename `orthogonal`→facet | No drift; but churns two skills + VRS contract and formalizes an edge set *ahead of* the tooling that enforces it (#1180) | **Passes** — the shared vocabulary is house-specific and prevents drift; keep the SKOS/OWL rationale as a one-line pointer, don't teach it |
| **3B — keep the five as lenses now; align names only when #1180 lands** | Fix only the clear bugs now (`set` split, leitwort attribution, "facet"); defer full edge-name unification to when the typed SoT exists | No premature churn; but skill/VRS/#1180 drift in the meantime (the exact risk the task flags) | Passes; minimal edits |
| **3C — minimal: add mapping table as reference, change nothing structural** | Paste the Tier-A/Tier-B mapping into the skill as a "lineage" aside | Cheapest; but a table the skill doesn't act on is frozen state → violates write-skill "no frozen state" | **Fails** the token bar — don't do 3C |

**Recommendation: 3A for the edge *names* + 3B's bug-fixes**, i.e. unify the
vocabulary now (it's cheap, purely a rename, and drift-prevention is the whole
point of doing this survey), but do **not** add resolver/tooling language to the
skill — that's #1180. Reject 3C.

### `/sk-ontology` — specific edits
Adopt-now (each ≤ a line or two, all pass the bar):
- Rename the "Name the edges" list to the unified vocabulary; mark the
  naming-relevant subset.
- Split `set` into subset(`isa`)/membership; delete the ambiguous "set" bullet.
- Rename `orthogonal`→**facet** with a one-clause nod ("independent axes, à la
  faceted classification"); keep the "names must not imply coupling" rule.
- One-line KOS-ladder framing for the "lazy structure" rule (flat = controlled
  vocabulary; +hierarchy = taxonomy; +related/alt = thesaurus; +typed edges =
  ontology).
Defer to #1180: typed source, resolver/compiler, `data.tql`/TypeDB, freshness
gate, integrity rules. Do **not** put these in the skill.
Cite-don't-teach: SKOS/OWL/DDD get pointer mentions only.

### `/sk-naming` — specific edits
- Nearly nothing to add — it already lands register/verb-noun/prior-art/alias
  hygiene, all of which restate knowledge Claude has; keep it lean.
- Optional one-liner: "prior-art borrowing" == DDD ubiquitous-language + SKOS
  prefLabel principle (borrow the domain's established word). Low value; include
  only if it earns the line.
- Cross-ref the facet rule for orthogonal axes ("don't let an identifier imply
  coupling between independent dimensions").
Everything else here **fails** the bar (Claude knows naming conventions).

### VRS `04-ontology` contract — specific edits
- Rename the relationship enumeration in `VRS.ONT-R06` to the unified vocabulary
  (keep it a *may*, keep lazy-structure).
- Add one line: downward inheritance == SKOS `inScheme` / concept-scheme
  boundaries (SOTA backing for the existing rule).
- The contract already forbids decisions/impl detail — no change needed there.
Defer to #1180: any typed-edge schema, `id`/`status` fields, machine-checkable
freshness — these are the epic's job, not the prose contract.

---

## 4. Where our current skills are wrong / redundant / misaligned

1. **`set` conflates subset and membership** (`/sk-ontology`, VRS `VRS.ONT-R06`).
   Subset ⊆ is subsumption (redundant with `hierarchy`/`isa`); membership ∈ is a
   distinct collection relation. SKOS/OWL keep these separate. **Fix:** drop
   "set"; fold subset into `isa`, name membership explicitly.
2. **Leitwort attribution** (`/sk-ontology`: "Term via Matt Pocock"). The term is
   from literary theory (Leitwortstil); Pocock applied it to skills. **Fix:**
   "from literary theory; applied to skills/engineering by Matt Pocock."
3. **"orthogonal" under-names a known concept.** It's **faceting** (Ranganathan;
   schema.org multi-domain). Borrow "facet" — the skill's own doctrine is "borrow
   the established word over inventing one," which it violates here.
4. **Vocabulary drift risk is real and current:** skill+VRS say
   {hierarchy/composition/orthogonal/set/layers}; #1180 says
   {isa/partOf/dependsOn/refines/altLabel/avoid}(+related). Two vocabularies for
   overlapping concepts, already diverging. This survey's core deliverable is to
   collapse them (§2).
5. **Redundancy hazard, not yet present but tempting:** do **not** import SKOS/OWL
   explanations into the skills. Claude knows them. The write-skill bar ("does
   Claude already know this?") kills any paragraph teaching broader/narrower or
   subClassOf. Keep them as one-line lineage pointers only.
6. **Not wrong, worth affirming:** our is-a / part-of split is *more* precise than
   SKOS and matches OWL/BFO — keep it; don't "simplify" toward SKOS's conflated
   `broader`.

## Conclusion

Unify the edge names and fix the ambiguous naming lenses now, while deferring
typed storage, resolution, and freshness machinery to `#1180`.

## VRS Impact

The result is reflected in `VRS.ONT-R06`: the ontology contract uses the shared
edge vocabulary, preserves the is-a/part-of distinction, and keeps structure
lazy. The survey remains evidence rather than a second normative contract.

---

## Sources
- SKOS Reference / Primer — https://www.w3.org/TR/skos-reference/ ,
  https://www.w3.org/TR/skos-primer/ ; SKOS overview —
  https://en.wikipedia.org/wiki/Simple_Knowledge_Organization_System
- KOS ladder — https://www.taxonomies-sig.org/about.htm ;
  http://accidental-taxonomist.blogspot.com/2020/12/differing-definitions-of-ontologies.html
- OWL / part-whole — https://www.w3.org/2001/sw/BestPractices/OEP/SimplePartWhole/ ;
  https://www.w3.org/TR/owl-guide/
- Faceted classification — http://archive.iainstitute.org/en/learn/research/a_simplified_model_for_facet_analysis.php ;
  https://www.britannica.com/science/facet-analysis
- schema.org philosophy — https://cacm.acm.org/practice/schema-org/
- BFO — https://en.wikipedia.org/wiki/Basic_Formal_Ontology ;
  http://ontology.buffalo.edu/smith/articles/Material_Entities.pdf
- DDD — Evans, Domain-Driven Design (fabiofumarola.github.io/nosql PDF) ;
  https://martinfowler.com/bliki/BoundedContext.html
- Leitwort — https://x.com/mattpocockuk/status/2066922013000671731 ;
  https://en.wikipedia.org/wiki/Leitmotif
- Naming conventions — https://learn.microsoft.com/en-us/dotnet/standard/design-guidelines/naming-guidelines ;
  https://google.aip.dev/190
