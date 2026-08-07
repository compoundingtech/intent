# VRS Reference — Spec

This document specifies `.reference/` directories. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Structure

```text
.reference/
  <source-or-topic>.md
```

Record format:

```markdown
# <Reference title>

Source: <URL, file, command, or system>
Captured: <date, when freshness matters>

## Relevant Facts

## VRS Impact
```

Use references for third-party API details, standards, external feature specs,
or source snapshots. Avoid copying large external documents; capture the facts
the VRS depends on and link to the source.

When a `requirements.md` constraint depends on external behavior, cite the
reference from the constraint:

```markdown
- **<NS>-C01 API rate limit:** Requests must stay within the provider's
  documented limit. Source: [.reference/provider-api.md](./.reference/provider-api.md).
```

The constraint is normative; the reference is evidence.
