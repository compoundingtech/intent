# Diagrams use full Mermaid; the ASCII previewer is advisory

Status: accepted

## Context

Intent and `context/` docs open non-trivial sections with the clearest structural
representation, often a Mermaid diagram (see [03-spec](../03-spec/spec.md)).
Diagrams have two render paths: an inline terminal/nvim ASCII preview
(`mermaid-ascii`, which parses only flowchart/graph and a minimal
sequenceDiagram) and full rendering via `mmdr` (all Mermaid diagram types).

The inline previewer rejects valid, widely-used Mermaid — `stateDiagram-v2`,
`alt`/`else` and other fragments, async/cross arrows, `Note` — and mis-renders
`{}` decision nodes. This forces a choice: constrain authored diagrams to the
previewer's subset, or author full Mermaid and treat the previewer as advisory.

## Evidence and Argument

A repo-wide `mermaid-lint` scan finds existing, correct diagrams (e.g. under
`context/scg`, `nixpkgs/home-manager/molty-docs`) that use `stateDiagram-v2`
and `alt` — unpreviewable in `mermaid-ascii` but rendered correctly by `mmdr`.
Constraining authoring to the previewer subset would ban state, class, ER, and
branching sequence diagrams from docs purely because of a downstream tool's
maturity. The cost of the alternative is small and bounded: the nvim plugin
shows a concise inline diagnostic naming the cause and pointing at `mmdr`, and
the warn-only lint surfaces the same repo-wide — so authors are never left
guessing why a block did not preview.

A tool gap should not become a documentation constraint. The durable fix is to
push the constraint upstream: file a clear reproduction against the previewer
and track it, so the gap closes and the constraint is temporary, rather than
degrading the docs to fit the weakest renderer.

## Options

| Option | Tradeoffs |
| --- | --- |
| Full Mermaid, advisory previewer | Docs use the latest/most expressive Mermaid; some blocks only render via `mmdr`. Requires a helpful inline error + a non-blocking lint so the gap is visible, and discipline to file upstream. |
| Subset-only hard gate | Every diagram previews inline; but state/class/ER/branching diagrams are banned from docs, and correct existing diagrams would have to be rewritten or removed. |
| No policy | Least effort now; authors hit silent non-rendering with no guidance and no path to closing the tool gap. |

## Decision

Diagrams may use the full, latest Mermaid syntax. The inline ASCII previewer
supports only a subset; blocks it cannot render are advisory, not errors:

- The nvim previewer shows an inline diagnostic in place of an unrenderable
  block (cause + every unsupported construct + the `mmdr` escape hatch).
- `check:docs:mermaid` is **warn-only** — it reports unpreviewable blocks
  repo-wide but never fails the build. `mmdr` is the full-render path.
- Previewer-tool gaps must not be worked around by degrading diagrams. Each gap
  is filed upstream with a clear reproduction and tracked until closed, so any
  constraint on supported syntax is temporary. The supported subset and the
  known gaps are recorded in `nix/scripts/mermaid-lint/unsupported-rules.json`.
