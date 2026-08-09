# Requirements: Intent CLI

## Context

This document defines testable constraints for the `intent` CLI: Intent hygiene,
graph extraction, and enforcement diagnostics. It builds on the enforcement contract in
[intent/16-enforcement](../../intent/16-enforcement/).

The `AXE.INTENT-*` requirement namespace is a stable document identifier carried
from the originating CLI contract; it does not name an executable command.

## Assumptions

- **A01 Intent owns semantics:** The root contract in [intent](../../intent/)
  owns artifact semantics, file contracts, review smells, and enforcement rules.
- **A02 CLI owns operator ergonomics:** `intent` gives humans and agents one
  discoverable command surface for Intent checks without making the CLI the
  Intent authority.
- **A03 Checks need multiple surfaces:** Agents need interactive diagnostics and
  JSON; CI needs deterministic exit codes; future planning needs evidence links.
- **A04 Markdown remains authoritative:** Early tooling parses Markdown-authored
  Intent artifacts and may emit derived graph JSON, but generated graph output is
  not the source of truth.
- **A05 Portable agent invocation:** Semantic Intent review can invoke a coding
  agent through the portable Coding Agent Invocation Contract from the
  originating repository instead of binding directly to a provider CLI.

## Acceptable Tradeoffs

- **T01 Shared CLI surface:** A top-level `intent` command minimizes indirection,
  while the reusable library keeps the checker available to other integrations
  without duplicating CLI behavior.
- **T02 Warning mode during migration:** Warning-mode checks are acceptable only
  as a calibrated migration state that later becomes blocking or is removed.
- **T03 Partial graph first:** The first graph model may cover only IDs, links,
  artifact ownership, and rule diagnostics rather than a complete typed Intent
  source model.

## Requirements

### Must Preserve Authority Boundaries

- **AXE.INTENT-R01 Intent authority:** `intent` must consume the root Intent contract; it
  must not define artifact semantics independently.
- **AXE.INTENT-R02 Checker engine boundary:** Deterministic parsing, graph
  extraction, rule evaluation, and diagnostics must live in a reusable checker
  engine that can be called by the CLI and by Nix checks.
- **AXE.INTENT-R03 No hidden rewrites:** `intent` must not silently rewrite Intent
  files. Any repair mode must be explicit, previewable, and scoped.
- **AXE.INTENT-R04 Nix owns blocking gates:** Merge-blocking enforcement runs
  through repo checks that call the same checker engine; the CLI is the operator UX,
  not the CI authority.

### Must Provide Agent-Friendly Commands

- **AXE.INTENT-R05 Check command:** `intent check [path]` must run deterministic
  Intent hygiene checks and return problems-first human output plus
  schema-versioned JSON.
- **AXE.INTENT-R06 Link command:** `intent links [path]` must inspect local
  Markdown links, mechanically resolvable anchors, wiki-style references, and
  broken or ambiguous targets.
- **AXE.INTENT-R07 ID command:** `intent ids [path]` must inspect ID definitions,
  declared scopes, duplicates, legacy syntax, bare-ID ambiguity, and cross-node
  reference ownership.
- **AXE.INTENT-R08 Graph command:** `intent graph [path] --json` must emit the
  derived Intent graph for tools and agents without treating that graph as
  authoritative source.
- **AXE.INTENT-R09 Doctor command:** `intent doctor` must summarize checker
  version, rule-set version, repository scope, migration status, allowlist
  status, and known blocking/warning counts.

### Must Support Enforcement Ratchets

- **AXE.INTENT-R10 Warning exit criteria:** Warning-mode rules must report the
  migration condition that prevents strict mode and the condition that promotes
  the rule to blocking or deletes it.
- **AXE.INTENT-R11 Rule stability:** Diagnostics must include stable rule IDs,
  severity, gate mode, artifact path, owner, evidence, and suggested fix.
- **AXE.INTENT-R12 Scoped execution:** Commands must support checking a specific
  Intent node, a subtree, or the whole repo so agents can run fast local checks
  while editing.
- **AXE.INTENT-R13 Evaluation feedback:** False positives and ambiguous diagnostics
  must be reproducible as isolated Intent evaluation fixtures before rules become
  blocking.

### Must Compose With Planning Later

- **AXE.INTENT-R14 Plan evidence:** Plan integration may attach Intent diagnostics and
  graph snapshots as plan evidence, but Plan must not own Intent checking
  semantics.
- **AXE.INTENT-R15 Proposed fixes:** Future planning or review workflows may carry
  proposed Intent patches, but accepted truth still lands in the owning Intent
  artifact.

### Must Support Semantic Review

- **AXE.INTENT-R16 Review command:** `intent review [path]` must run semantic Intent
  review through the portable Coding Agent Invocation Contract rather than
  binding to a provider-specific CLI. The command must fail before provider
  invocation when it detects CI or another known automated environment.
- **AXE.INTENT-R17 Baked review prompt:** The Intent semantic review prompt and output
  schema must be owned by Intent/CLI docs and eval-tuned; callers must not provide
  arbitrary prompt files for this command.
- **AXE.INTENT-R18 Review-only safety:** `intent review` must invoke the coding
  agent in read-only review mode by default and must not mutate Intent artifacts.
- **AXE.INTENT-R19 Fixture review grading:** `intent review-fixtures [path]` must
  run semantic review against an isolated materialized copy of each selected
  semantic-review fixture and grade the returned result against that fixture's
  minimum finding assertions by `rule`, `severity`, `artifact`, and `owner`. It
  must not require matching summary, evidence, or suggested-fix wording, must
  report which assertions were unmet, and must obey the same automated-context
  refusal as `intent review` without adding a token-spend flag.
