# Link Structure and Commit-Scoped References

## Question

What reference structure should VRS prefer for cross-references that must remain
internally consistent within one repository commit: relative Markdown links plus
scoped IDs, wiki-style links, or symbolic references backed by a resolver?

## Method

Three isolated probes compared existing repository usage and temporary refactor
scenarios:

- scan existing VRS-heavy trees for Markdown links, wiki links, and ID-like
  references;
- simulate a VRS split/move/renumbering under `/tmp`;
- compare longer-term structured representations such as Markdown conventions,
  sidecar metadata, generated Markdown, and typed Markdown parsing.

## Result

Relative Markdown links dominate current VRS-like docs. In the sampled
VRS-heavy areas, Markdown links substantially outnumbered wiki links, while
ID-like references were already common.

Wiki-style links were concentrated in a smaller set of docs and became
ambiguous in hierarchical VRS scenarios because VRS intentionally repeats file
names such as `requirements.md` and `spec.md` across child nodes.

The refactor simulation found that symbolic or namespaced IDs backed by a
resolver or registry made moving, splitting, and renumbering easier than
path-only references. Plain Markdown links remained useful for navigation but
required manual updates after moves.

The structured-representation probe favored a typed Markdown hybrid: Markdown
remains the authored medium, while tooling parses VRS artifacts into a typed
graph for validation, backlinks, rename plans, and optional projections.

## Conclusion

Canonical VRS references should be stricter than optional wiki links:

- use scoped, symbolic, or namespaced IDs for clause identity within the current
  commit;
- use relative Markdown links for concrete file or artifact navigation;
- do not treat wiki-style links as canonical normative references unless a
  resolver can prove they are unique and unambiguous.

Longer term, VRS should move toward a typed Markdown graph rather than a loose
collection of Markdown conventions.

## VRS Impact

Supports the roadmap entry for structured VRS representation in
[roadmap.md](../roadmap.md). It should inform future updates to
[02-requirements](../02-requirements/spec.md),
[03-spec](../03-spec/spec.md), and
[16-enforcement](../16-enforcement/spec.md) once the strict reference rule is
accepted.
