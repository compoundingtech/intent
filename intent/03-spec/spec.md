# VRS Spec — Spec

This document specifies `spec.md` files. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Structure

```markdown
# <Topic> — Spec

This document specifies ... It builds on [requirements.md](./requirements.md).

## Status

## Scope

## <Subsystem or Mechanism>
```

Each non-trivial section opens with the clearest structural representation:
ASCII diagram, Mermaid diagram, table, schema, or numbered flow.

Mermaid diagrams may use the full, latest syntax. The inline ASCII previewer
(`mermaid-ascii`) renders only a subset (flowchart/graph and a minimal
sequenceDiagram); blocks it cannot render are advisory, not errors, and render
fully with `mmdr`. Authoring is not constrained to the preview subset — see
[.decisions/0031](../.decisions/0031-diagrams-use-full-mermaid-previewer-advisory.md).
Previewer gaps are filed upstream and tracked, not worked around in the docs.

Use design-question IDs (`DQ1`, `DQ2`) for unresolved choices. When a question
is resolved, remove the question and update the relevant spec, requirement,
decision record, or experiment.

Requirement references in specs are commit-scoped consistency checks, not
long-term stable API. A spec may refer to local IDs such as `R03` when it builds
on one clearly linked `requirements.md`; cross-node references should include a
namespace or Markdown link to the owning requirements artifact.
