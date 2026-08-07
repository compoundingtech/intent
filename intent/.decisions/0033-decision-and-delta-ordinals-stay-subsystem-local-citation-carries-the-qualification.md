# Decision and delta ordinals stay subsystem-local; the citation carries the qualification

Status: accepted

Scope: repo-wide, every `.decisions/` and `.delta/` directory under `context/`.
Resolves the id-uniqueness defect recorded in
`context/coding-agents/open-questions.md` in the originating `schickling/dotfiles`
repository, which left open the choice between a global prefix registry and a
citation rule.

## Context

Requirement ids are globally unique because
coding-agents 0093 (`context/coding-agents/.decisions/0093-requirement-ids-are-globally-unique-prefixed.md`
in `schickling/dotfiles`) registers a per-subsystem prefix (`RT-`, `AP-`, `CAIC-`). Decisions and deltas have
no such registry, so a bare ordinal is a homonym: `0003` cited from outside its
subsystem names a record in **34** different `.decisions/` directories, `0007` in 20,
and the most-collided delta ordinal in **25**. This has already cost live reasoning —
two agents ran a full correction-and-retraction cycle over a handoff item because a
delta ordinal collided seven ways and resolved three of them in context, and the
citation that overturned a stated design direction was itself one of the ambiguous
ordinals. (Those ordinals are deliberately
not written bare here: by this record's own rule they would name nothing.)

## Evidence and Argument

The obvious fix — prefix every filename, `AX.0021` — was rejected. It makes a local
artifact carry global identity, costs several hundred renames plus a citation sweep,
and, decisively, **does not fix the defect**: nothing stops two `AX.0021` files any
more than two `0021` files. Uniqueness is an *allocation* property, not a naming one.

Measurement bears that out. The tree already contains 13 directories where one
ordinal names two files, so the premise that "a subsystem's local ordinal is
unambiguous, the way a local variable is" was **not true** when it was assumed. It
has to be enforced before it can be relied on.

**The mechanism is concurrent allocation, not careless naming.** The live duplicate ordinal
under coding-agents was traced to its origin: the two records were created on
divergent branches (`4eac37695` and `3666c2727`, neither an ancestor of the other),
and *neither branch had that ordinal at its own parent*. Both authors looked, saw the
ordinal free, and took it; the collision came into existence at the merge. Next-free-
ordinal allocation has no concurrency control, so parallel branches will reliably
collide no matter how well citations are written. This is why the fix has to include
an allocation rule — a pure citation convention would leave the mechanism running.

## Options

| Option | Disposition |
| --- | --- |
| Globally unique, prefixed decision and delta ordinals | Rejected. It imposes a global identity and a repo-wide rename/citation sweep without enforcing exclusive allocation of a prefixed ordinal. |
| Keep subsystem-local ordinals and rely only on qualified citations | Rejected. Qualification disambiguates cross-subsystem references but leaves concurrent same-directory allocation collisions possible. |
| Enforce subsystem-local allocation and require qualification at the citation boundary | Accepted. ALLOC makes the local ordinal unique; CITE and LABEL make references resolvable without turning local records into global identities. |

## Decision

A bare ordinal is a subsystem-local name. Reading it as an identifier is legitimate
only inside its own subsystem, and only if allocation keeps it unique there. Three
rules make that true; `nix/scripts/vrs-id-check/vrs-id-check.ts` enforces all three
against a ratcheting baseline, the same idiom as the link-resolution gate.

**ALLOC — one ordinal, one file, per directory.** Within a `.decisions/` or `.delta/`
directory an ordinal names exactly one record. This is the rule that makes the other
two meaningful, and the one that prefixing could never provide. It also stops slot
recycling: a resolved delta whose file is deleted must not have its ordinal reissued,
because every existing citation silently repoints to the new record.

**Allocation is monotonic — gaps are never refilled.** A hole in a directory's
numbering marks a retired or renamed record, and inbound citations to it may still
exist, including bare ones no gate can see. Taking the next ordinal above the highest
used, rather than the lowest free one, is what keeps those citations from silently
resolving to something unrelated. Contiguous numbering is not a goal; a gap is
evidence, and monotonic allocation is also the only form that stays safe when two
branches allocate at once.

**Audit gaps by definition, not by token.** A gap is an *absent definition*, not an
absent mention. A well-kept tree records why an id was retired, and that note has to
name the id — so the better the gap is documented, the more certainly a token scan
finds it "present". Verifying this rule on the corroborating file below, a scan for
id tokens returns a contiguous run and the confident, wrong conclusion that it has no
gaps; only a scan for *defined* ids shows the three holes, which survive as references
solely inside the note explaining their own absence. The obvious check runs clean and
answers backwards, so compliance must be audited against definitions.

The same call was already made independently, for requirement ids and in a different
tree, before this record existed:
the historical 06-smalltalk-transport requirements
(`context/coding-agents/06-smalltalk-transport/requirements.md` in
`schickling/dotfiles` at commit `b299be3c4869554e7c24ac66ddd507aa8a18abbb`)
moved two requirements out and left their numbers as holes, noting that "ids are
stable identifiers here, and reusing `R08` for a different requirement would silently
redirect any surviving citation." That is this rule's reasoning arrived at from the
other direction, which is better evidence for it than the argument above.

**CITE — a bare ordinal must resolve locally.** Legal if the record lives in the
citing document's own subsystem or an **ancestor** subsystem (a nested subsystem may
cite its parent's records bare — it is inside that scope). A reference resolving
nowhere locally is either stale or an unqualified cross-subsystem citation, and must
be written as a path-qualified markdown link.

**First qualified use establishes the bare form.** A document that links a record
once may use the bare ordinal for the rest of the passage. This is ordinary prose
citation practice and already the house style; the qualified form is present, so the
reader can resolve it. The gate honours it rather than demanding link spam.

**LABEL — a link's visible ordinal must match the record it points at.** A link whose
label shows one ordinal while its target filename carries another
is invisible to the link-resolution gate, because the target resolves; the citation
nonetheless names a different record than it links.

**Nearest owner wins.** Subsystems restart numbering at `0001`, so an ordinal is
routinely owned by both a subsystem and one of its ancestors — 220 such shadowing
relationships across the tree. That shadowing is normal and
not a defect: a bare ordinal reads as the nearest owner, the way lexical scope works.

The consequence is a rule rather than a check — **cross-cutting decisions that get
cited from all over should always be linked, never written bare**, because a bare cite
resolves nearest-first and will silently pick up a local homonym in any subsystem that
happens to own that ordinal. This is undetectable by the same logic that makes a
recycled slot undetectable: the reference resolves, it just resolves to the wrong
record.

That 220 needs its counting convention stated, because plausible ways of counting
shadowing differ by more than 5x — an independent re-measurement produced three
different figures before matching. **It counts (descendant subsystem, ordinal,
ancestor subsystem) shadowing relationships.** The same tree gives 39 distinct
*ordinals* standing in some ancestor relation, 188 shadowed (subsystem, ordinal)
pairs, 105 ordinals owned by more than one directory whether or not they are
ancestor-related, and 243 of 963 records sitting on one side of a shadow. Any is
defensible; none is interchangeable, and the smallest is the one a reader is most
likely to assume from the word "ordinals".

**Prefer moving the record whose citations are checkable.** When a collision must be
broken by renumbering one of the two, count *kinds* of inbound citation, not inbound
citations. A path-qualified link that breaks fails loudly at the link gate; a bare
ordinal left behind silently resolves to the survivor. So the safe file to move is
often the one with *more* references, if those references are links. The live
coding-agents collision is exactly this shape: the more-cited record is the safer one
to move, because the less-cited one is the referent of seven bare ordinals in an
`ontology.md` that no gate can see.

## Consequences

- **ALLOC and CITE are coupled, and the coupling is load-bearing.** CITE tests set
  membership, so it cannot tell one owning file from two: its guarantee that a bare
  ordinal is unambiguous holds only for ordinals that pass ALLOC. The classes are
  assigned so that together they are complete — ALLOC owns "resolves to more than one
  record locally", CITE owns "resolves to none".
- **One gap neither rule closes: a citation to a recycled slot that still resolves.**
  If an ordinal is reissued, a bare reference to the old record resolves cleanly to
  the new one and no rule fires. ALLOC prevents this going forward by refusing the
  duplicate at allocation; it cannot detect one already absorbed. The known instance
  is corrected in
  coding-agents 0130
  (`context/coding-agents/.decisions/0130-deploy-is-enforced-at-the-nix-and-system-switch-seams-and-hygiene-rails-stay-persona-agnostic.md`
  in `schickling/dotfiles`), which cited a delta ordinal whose slot had since been
  reused.
- **This does not foreclose a prefix registry.** If decisions later need global
  identity for reasons other than uniqueness, ALLOC is the precondition for it, not a
  competitor to it.
- **Why this is a gate and not a convention.** The observability collisions were
  already noticed and carefully documented in prose — see the closing note of
  observability 0024
  (`context/observability/.decisions/0024-dashboard-ownership-source-colocation.md`
  in `schickling/dotfiles`),
  which names all four duplicated ordinals, explains the two families that produced
  them, and defers the fix to "a separate housekeeping pass." That pass never
  happened, and the collisions are still here. A convention that is written down but
  unenforced is how this defect accumulated; recording it again would repeat the
  mistake rather than fix it.
- Pre-existing violations are baselined as recorded debt, so the gate lands green and
  fails only on new breakage. The baseline is read entry-by-entry, never refreshed
  blind: the tail is 13 ALLOC collisions and a set of CITE references, of which the
  largest single group is SCG decisions citing delta ordinals that were never
  materialized as files.
