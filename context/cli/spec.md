# Spec: Intent CLI

This document specifies the `intent` command realization. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Scope

Defines:

- `intent` command surface;
- routing between the CLI, the Intent checker engine, Nix checks, and future Plan
  evidence;
- output schema and migration-ratchet behavior for deterministic Intent checks;
- semantic review routing through the portable Coding Agent Invocation Contract.

Does not define:

- Intent artifact semantics, file contracts, review smells, or enforcement rules;
  see [intent](../../intent/) and
  [intent/16-enforcement](../../intent/16-enforcement/);
- semantic LLM review rubric content beyond routing diagnostics;
- future typed-source Intent authoring;
- Plan storage or workflow semantics.

## Architecture

```text
human / coding agent / repo check
  -> intent
       -> intent-check engine
            -> parse Markdown Intent artifacts
            -> derive graph view
            -> run deterministic rules
            -> emit diagnostics

Nix checks
  -> intent-check engine
       -> blocking exit code for strict rules

intent review
  -> intent-check engine diagnostics
  -> Coding Agent Invocation Contract
       -> provider backend in read-only review mode
       -> schema-validated semantic findings
```

`intent` is an operator and agent UX. The checker engine is the reusable
implementation boundary. The root Intent contract remains the semantic authority.

## Command Surface

```text
intent check [path] [--profile local|strict] [--warnings-as-errors] [--json]
intent graph [path] [--json]
intent review [path] [--profile local|strict] [--backend <id>] [--coding-agent <path>] [--report <path>]
intent review-fixtures [path] [--fixture <id>]... [--backend <id>] [--coding-agent <path>] [--json] [--report <path>]
```

Default `path` is the current directory, and
`./15-evaluation/semantic-review` for `review-fixtures`. The first
implemented surfaces are `check`, `graph`, `review`, and `review-fixtures`;
`links`, `ids`, and `doctor` remain target surfaces for later implementation
slices. Commands discover Intent nodes by walking for
`requirements.md`, `spec.md`, known companion directories, and child node shapes
supported by the root Intent contract.

## Authority Routing

| Command | Route | Authority |
| --- | --- | --- |
| `intent check` | checker engine deterministic rules | Root Intent contract defines rules; checker evaluates |
| `intent links` | checker engine link pass | Git working tree/current commit |
| `intent ids` | checker engine ID pass | Root Intent identifier contract |
| `intent graph` | checker engine graph extraction | Markdown Intent artifacts are source; graph is derived |
| `intent review` | checker diagnostics plus `$CODING_AGENT` review invocation | Intent/CLI own prompt and schema; Coding Agent Invocation Contract owns provider portability |
| `intent review-fixtures` | same review invocation over a materialized fixture copy, graded against fixture assertions | Intent evaluation owns fixtures and assertion semantics; the CLI owns the runner |
| `intent doctor` | CLI plus checker metadata | CLI for UX; checker for rule/migration state |
| Nix check | checker engine strict profile | Nix check is blocking gate |

## Checker Engine Boundary

The checker engine should be callable without the `intent` CLI. It owns:

- filesystem discovery of Intent nodes;
- Markdown parsing needed for deterministic rules;
- ID definition and reference extraction;
- local Markdown link and anchor resolution;
- wiki-link classification;
- `.delta/`, `.decisions/`, `.experiments/`, and `.reference/` shape checks;
- derived graph JSON;
- diagnostic JSON.

The CLI owns:

- subcommand and argument parsing;
- problems-first human rendering;
- schema-versioned machine output;
- exit-code mapping.

Nix owns:

- strict rule profile selection;
- merge-blocking check derivations;
- reproducible local/CI execution.

The Coding Agent Invocation Contract owns provider-portable non-interactive
agent invocation. `intent review` consumes it; it does not call provider CLIs
directly.

## Implementation Order

The first `intent` implementation slice should build deterministic enforcement
primitives before semantic review:

1. checker engine boundary and filesystem discovery;
2. `check` and diagnostic JSON;
3. Nix strict-check integration over the same checker;
4. `graph` for the mechanically derived v0 subset;
5. `links` and `ids` for dedicated drill-down views over the same extracted
   facts;
6. `review` through CAIC after deterministic diagnostics are stable.

`intent review` remains part of the public target surface, but it should not be
implemented before the deterministic checker can produce bounded, schema-shaped
diagnostics for the review packet.

## Diagnostic Schema

Every command that reports findings emits the same diagnostic object in JSON:

```json
{
  "schema_version": "axe.intent.diagnostic.v1",
  "rule": "INTENT.ENF.link.local-target",
  "severity": "warning",
  "gate": "transitional | blocking | advisory | review",
  "artifact": "context/intent/spec.md",
  "owner": "16-enforcement",
  "range": { "line": 42, "column": 1 },
  "evidence": "Link target does not exist: ./missing.md",
  "suggested_fix": "Update the Markdown link or create the referenced artifact.",
  "migration": {
    "state": "warning",
    "exit_condition": "Known broken links are fixed or allowlisted."
  }
}
```

Human output groups diagnostics by severity and owner, with blocking findings
first, then transitional warnings, then advisory findings. It does not print
large graph payloads unless `--json` is requested.

## Rule Profiles

| Profile | Use | Behavior |
| --- | --- | --- |
| `local` | Interactive editing | Blocking rules fail; warnings render but exit success unless `--warnings-as-errors` |
| `strict` | Nix check / CI | Blocking rules fail; migrated warning classes are promoted to blocking |
| `migration` | Rule development | Emits all known transitional findings with exit criteria |

The rule profile is part of the diagnostic envelope so agents can tell whether a
finding is a local advisory, a migration warning, or a merge blocker.

## Initial Rule Set

The first implementation calls a narrow subset of root Intent enforcement rules:

1. local Markdown `.md` link existence;
2. strict decision-record shape for `context/intent/.decisions/`.

Local link findings are transitional warnings in the `local` profile and
blocking in the `strict` profile once the known corpus is clean. Root Intent
decision-shape findings are blocking for the root contract because its current
decisions have been migrated to that shape. ID, wiki-link, delta, and
graph rules remain future deterministic primitives and must carry exit criteria
when introduced.

## Derived Graph

`intent graph --json` emits the mechanically extracted, resolvable subset of
the Intent graph with a deliberately small v0 shape:

```json
{
  "schema_version": "axe.intent.graph.v0",
  "root": "/repo/context/intent",
  "nodes": [
    {
      "id": "file:requirements.md",
      "kind": "file",
      "title": "requirements.md",
      "path": "requirements.md",
      "status": "active",
      "refs": [],
      "refines": [],
      "evidence": []
    },
    {
      "id": "INTENT-R27",
      "kind": "requirement",
      "title": "Enforcement",
      "path": "context/intent/requirements.md",
      "status": "active",
      "refs": [],
      "refines": [],
      "evidence": []
    }
  ],
  "edges": [
    {
      "source": "file:requirements.md",
      "target": "INTENT-R27",
      "kind": "contains",
      "path": "requirements.md",
      "evidence": "structured-id"
    },
    {
      "source": "file:spec.md",
      "target": "file:requirements.md",
      "kind": "markdown_link",
      "path": "spec.md",
      "evidence": "./requirements.md"
    }
  ]
}
```

Graph v0 extracts:

| Fact | Representation |
| --- | --- |
| Markdown artifact | `file:<relative path>` node with `kind: "file"` |
| Bold structured ID such as `**AXE.INTENT-R08 Graph command:**` | ID node plus `contains` edge from the artifact |
| Local resolvable Markdown link to another Markdown artifact under the graph root | `markdown_link` edge between file nodes |
| Wiki-style reference such as `[[Graph Backlog]]` | `wiki:<target>` node with `status: "unresolved"` plus `wikilink` edge |
| Inline `refines: <ID>` text on a structured-ID line | `refines[]` on that ID node |

The graph is derived from Markdown and checker-owned parsing rules. It is not a
sidecar source of truth and is not expected to cover every semantic relationship
in early versions. If a graph fact cannot be inferred reliably, the checker
emits an ambiguous-structure diagnostic rather than inventing a node or edge.

## Semantic Review

`intent review` runs semantic review after deterministic checks. It builds a
bounded review packet from:

- the baked Intent semantic-review prompt owned at
  [intent/16-enforcement/review-prompt.md](../../intent/16-enforcement/review-prompt.md);
- the target Intent files;
- `intent check --json` diagnostics;
- the semantic review output schema at
  [intent/16-enforcement/review-result.schema.json](../../intent/16-enforcement/review-result.schema.json).

The command invokes `$CODING_AGENT` through the Coding Agent Invocation Contract
(specified in `context/coding-agents/22-invocation-contract/spec.md` in the
originating `schickling/dotfiles` repository):

```text
$CODING_AGENT run \
  --cwd <repo-root> \
  --prompt-file <baked-intent-review-prompt> \
  --context-file normative:<target-intent-file>... \
  --context-file generated-diagnostics:<intent-check-json> \
  --mode review \
  --permission read-only \
  --approval never \
  --config-policy isolated \
  --network-policy disabled \
  --output-format json \
  --output-schema <intent-review-result-schema>
```

Callers do not provide an arbitrary prompt. Prompt and schema changes happen in
the Intent enforcement node and are validated through eval fixtures before becoming
the baked review version.

Provider readiness is checked before spending model tokens. `intent review`
first runs `$CODING_AGENT capabilities --json`, resolves `--backend <id>` or the
advertised `default_backend`, and validates that backend against the review
contract. It does not invoke `$CODING_AGENT run` until the selected backend is
known to the adapter and supports all of the requested contract pieces:

| Capability | Required value for `intent review` |
| --- | --- |
| mode | `review` |
| permission | `read-only` with provider-native, agent-policy, or adapter-sandbox enforcement |
| approval | `never`, fail-closed before provider execution if the backend may prompt |
| config policy | `isolated` |
| network policy | `disabled` for agent/tool/web access; provider model transport may still be required |
| output | `json` plus schema validation for `axe.intent.review.v1` |

Known backend support is capability-based, not provider-name-based. If the
default backend or a `--backend <id>` override is unknown, unavailable, missing
one of these capabilities, or only supports a weaker permission/config/output
contract, `intent review` fails before invoking the provider. The supported
backend set may expand without changing the Intent CLI contract as long as the
backend satisfies these same readiness criteria.

Production readiness is backend-scoped. A backend is ready for `intent review`
when its capability preflight passes, token-free fake-provider regression tests
cover its provider-specific command mapping, and at least one bounded manual
real-provider run against a small existing Intent subsystem has produced a
schema-valid `axe.intent.review.v1` report. One backend's pending real-provider
evidence does not block another backend that satisfies the same readiness
criteria.

Semantic findings use the diagnostic shape where possible, but their gate is
`review` unless a later Intent decision makes a specific semantic rule blocking.
Review mode must not write files or silently apply fixes.

Any `intent review` or fixture-review mode that calls a real provider and
spends model tokens is manual-only. It must not be part of CI, Nix checks,
pre-commit hooks, scheduled jobs, or default automated validation. Automated
validation may cover deterministic fixture shape and fake-provider invocation
only. The command does not require an additional token-spend confirmation flag;
invoking `intent review` is the explicit operator action. The mechanical guard
is that the command fails before provider invocation when it detects CI or other
known automated environments.

Output defaults to stdout. On success, stdout contains the CAIC JSON result
envelope whose `result` field conforms to `axe.intent.review.v1`; provider progress
and human logs remain on stderr. `--report <path>` writes the same final JSON
envelope to a caller-chosen path and leaves stdout empty on success, matching
CAIC `--final-output` semantics. Tool or schema failures use the CAIC error
envelope when available; task findings remain structured review findings and do
not imply a nonzero exit code unless a future explicit fail policy is added.

## Fixture Review Grading

`intent review-fixtures` is the runner for the semantic-review fixtures owned by
[intent/15-evaluation](../../intent/15-evaluation/spec.md). It exists so the
minimum-assertion contract is executable rather than descriptive.

For each selected fixture it:

1. reads the fixture manifest and resolves the fixture's declared `prompt_ref`
   and `schema_ref`, so the graded run uses the assets the fixture claims;
2. materializes the fixture's `input/` tree into an isolated temporary
   workspace, leaving the tracked corpus untouched;
3. runs the same review invocation as `intent review` with that workspace as
   both the review root and the coding-agent working directory, which makes the
   fixture-relative artifact paths in `assertions.json` the paths a review
   result reports;
4. grades the returned `axe.intent.review.v1` result against
   `assertions.json`: every `minimum_findings` entry must be satisfied by some
   finding with the same `rule`, `severity`, `artifact`, and `owner`.

Summary, evidence, and suggested-fix wording are never compared. Artifact paths
are compared exactly after normalizing a workspace-absolute path to its
workspace-relative form; no suffix matching, so a finding routed to the wrong
artifact fails. A fixture without `assertions.json` is reported as skipped
rather than passed.

The command emits an `axe.intent.review-fixtures.v1` grading report with per-fixture
`matched` and `missing` assertions. Exit code 0 means every graded assertion was
met, 1 means at least one fixture had unmet assertions, and 2 means a fixture
review could not be run at all.

Fixture review spends model tokens whenever it is pointed at a real backend, so
it carries the same manual-only policy and the same automated-context refusal as
`intent review`, and it adds no token-spend flag. Automated coverage uses
fake/probe providers.

## Failure Behavior

| Failure | Behavior |
| --- | --- |
| Path is outside repo | fail with path diagnostic |
| No Intent node found | report no-scope diagnostic, exit nonzero for `check` |
| Markdown parse issue | report artifact diagnostic with line evidence |
| Broken link in strict profile | exit nonzero |
| Transitional warning | render warning and exit success unless `--warnings-as-errors` |
| `$CODING_AGENT` missing for review | fail with review-tool diagnostic |
| Selected backend is unknown, unavailable, or lacks review/read-only/isolated/schema/approval-never support | fail before provider invocation with review-tool diagnostic |
| Review output fails schema | fail with review-tool diagnostic |
| Review would spend real model tokens from CI or another automatic gate | fail with review-tool diagnostic |
| Fixture review cannot resolve a fixture, its assets, or its result | report the fixture as errored and exit nonzero |
| Checker internal error | fail closed with tool diagnostic |

## Plan Integration Boundary

Plan may record:

- `intent check --json` output as evidence;
- proposed Intent patches as review/proposed-patch records;
- resolved durable learning routed to Intent artifacts.

Plan must not become the checker. It records evidence and work coordination
around Intent changes; `intent-check` and the root Intent contract keep semantic
ownership.

## Anti-Goals

- Do not create a second CLI separate from `intent` without evidence that users
  need one.
- Do not make the CLI the authority for Intent semantics.
- Do not make generated graph JSON authoritative.
- Do not make warning-mode rules permanent.
- Do not silently rewrite Intent artifacts.
