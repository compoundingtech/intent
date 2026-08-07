# VRS Ontology — Spec

This document specifies `ontology.md` files. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Structure

```markdown
# <Topic> — Ontology

One or two sentences describing the scope.

## Language

**Term**:
One or two sentences defining the concept.
_Avoid_: Alias, misleading synonym

## Structure

<optional> How the terms relate — hierarchy, composition, orthogonality (⟂),
set membership, layers — and the leitwort each family carries. Prose, a list,
or a diagram. Omit when the term list has no structure worth stating.
```

The `## Language` section is required; a flat language list is the minimal
ontology (what was formerly a glossary). Add `## Structure` only where a
relationship carries weight — see `/sk-ontology` for the design discipline.

Add `## Flagged ambiguities` when the same word carries multiple concepts —
within this file, as a child narrowing a parent term, or as a cross-node
homograph (the same word owned by another ontology).

Inheritance flows downward, along concept-scheme boundaries: a child ontology
names the parent it inherits (VRS.ONT-R08) and defines only new terms, new
structure, or local ambiguity; it does not restate the parent.
