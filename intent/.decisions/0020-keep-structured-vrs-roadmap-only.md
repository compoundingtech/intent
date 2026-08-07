# Keep structured VRS roadmap-only for now

Status: accepted

## Context

VRS currently uses Markdown files plus deterministic conventions. The roadmap
already identifies a future structured intent layer with symbolic references,
typed records, checked schemas, and better refactoring support. The open design
question is whether to introduce machine-readable structure now or keep it as a
future direction until enforcement proves the need.

## Evidence and Argument

The current implementation path is still centered on Markdown-authored VRS
artifacts and a narrow deterministic enforcement subset. The immediate checker
can extract useful facts from files, links, headings, IDs, and companion
directory shapes without making authors maintain frontmatter, sidecars, or typed
source files.

Adding structure too early would create migration and authoring overhead before
the checker has demonstrated which facts it cannot infer reliably. Keeping the
structured layer on the roadmap preserves the direction while letting evidence
from `axe vrs` enforcement and isolated evaluations decide the next step.

## Options

| Option | Tradeoffs |
| --- | --- |
| Roadmap only | Lowest authoring friction and preserves Markdown as the normative source, but restructuring and renumbering remain brittle until tooling matures. |
| Add light frontmatter soon | Gives tooling explicit artifact metadata, but risks two sources of shape truth and premature schema churn. |
| Add sidecar graph files | Improves symbolic tooling, but authored sidecars can drift unless generated-only rules are strict. |
| Move to typed VRS source | Strongest long-term tooling model, but highest migration cost and premature for the current checker maturity. |

## Decision

Keep structured VRS representation roadmap-only for now. Markdown VRS artifacts
remain normative. Deterministic enforcement should extract a narrow derived
graph from Markdown and report ambiguous facts instead of requiring frontmatter,
sidecars, or typed source in the current contract.

Revisit this decision when `axe vrs` enforcement repeatedly needs facts that
cannot be inferred reliably from Markdown links, headings, IDs, and companion
artifact shapes.
