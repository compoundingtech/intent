# Requirements: Axe VRS

## Context

This document defines testable constraints for `axe vrs`, the Axe command family
for VRS hygiene, graph extraction, and enforcement diagnostics. It builds on
[../requirements.md](../requirements.md) and the meta-VRS enforcement contract in
[context/vrs/16-enforcement](../../../vrs/16-enforcement/).

## Assumptions

- **A01 VRS owns semantics:** The meta-VRS in [context/vrs](../../../vrs/)
  owns artifact semantics, file contracts, review smells, and enforcement rules.
- **A02 Axe owns operator ergonomics:** `axe vrs` gives humans and agents one
  discoverable command surface for VRS checks without making Axe the VRS
  authority.
- **A03 Checks need multiple surfaces:** Agents need interactive diagnostics and
  JSON; CI needs deterministic exit codes; future planning needs evidence links.
- **A04 Markdown remains authoritative:** Early tooling parses Markdown-authored
  VRS artifacts and may emit derived graph JSON, but generated graph output is
  not the source of truth.
- **A05 Portable agent invocation:** Semantic VRS review can invoke a coding
  agent through the portable
  [Coding Agent Invocation Contract](../../22-invocation-contract/requirements.md)
  instead of binding directly to a provider CLI.

## Acceptable Tradeoffs

- **T01 Axe subcommand over top-level CLI:** Using `axe vrs` is acceptable
  because coding agents already use Axe as the operator surface, but the checker
  engine must remain separable from Axe command parsing.
- **T02 Warning mode during migration:** Warning-mode checks are acceptable only
  as a calibrated migration state that later becomes blocking or is removed.
- **T03 Partial graph first:** The first graph model may cover only IDs, links,
  artifact ownership, and rule diagnostics rather than a complete typed VRS
  source model.

## Requirements

### Must Preserve Authority Boundaries

- **AXE.VRS-R01 VRS authority:** `axe vrs` must consume the meta-VRS contract; it
  must not define artifact semantics independently.
- **AXE.VRS-R02 Checker engine boundary:** Deterministic parsing, graph
  extraction, rule evaluation, and diagnostics must live in a reusable checker
  engine that can be called by Axe and by Nix checks.
- **AXE.VRS-R03 No hidden rewrites:** `axe vrs` must not silently rewrite VRS
  files. Any repair mode must be explicit, previewable, and scoped.
- **AXE.VRS-R04 Nix owns blocking gates:** Merge-blocking enforcement runs
  through repo checks that call the same checker engine; Axe is the operator UX,
  not the CI authority.

### Must Provide Agent-Friendly Commands

- **AXE.VRS-R05 Check command:** `axe vrs check [path]` must run deterministic
  VRS hygiene checks and return problems-first human output plus
  schema-versioned JSON.
- **AXE.VRS-R06 Link command:** `axe vrs links [path]` must inspect local
  Markdown links, mechanically resolvable anchors, wiki-style references, and
  broken or ambiguous targets.
- **AXE.VRS-R07 ID command:** `axe vrs ids [path]` must inspect ID definitions,
  declared scopes, duplicates, legacy syntax, bare-ID ambiguity, and cross-node
  reference ownership.
- **AXE.VRS-R08 Graph command:** `axe vrs graph [path] --json` must emit the
  derived VRS graph for tools and agents without treating that graph as
  authoritative source.
- **AXE.VRS-R09 Doctor command:** `axe vrs doctor` must summarize checker
  version, rule-set version, repository scope, migration status, allowlist
  status, and known blocking/warning counts.

### Must Support Enforcement Ratchets

- **AXE.VRS-R10 Warning exit criteria:** Warning-mode rules must report the
  migration condition that prevents strict mode and the condition that promotes
  the rule to blocking or deletes it.
- **AXE.VRS-R11 Rule stability:** Diagnostics must include stable rule IDs,
  severity, gate mode, artifact path, owner, evidence, and suggested fix.
- **AXE.VRS-R12 Scoped execution:** Commands must support checking a specific
  VRS node, a subtree, or the whole repo so agents can run fast local checks
  while editing.
- **AXE.VRS-R13 Evaluation feedback:** False positives and ambiguous diagnostics
  must be reproducible as isolated VRS evaluation fixtures before rules become
  blocking.

### Must Compose With Planning Later

- **AXE.VRS-R14 Plan evidence:** Plan integration may attach VRS diagnostics and
  graph snapshots as plan evidence, but Plan must not own VRS checking
  semantics.
- **AXE.VRS-R15 Proposed fixes:** Future planning or review workflows may carry
  proposed VRS patches, but accepted truth still lands in the owning VRS
  artifact.

### Must Support Semantic Review

- **AXE.VRS-R16 Review command:** `axe vrs review [path]` must run semantic VRS
  review through the portable Coding Agent Invocation Contract rather than
  binding to a provider-specific CLI. The command must fail before provider
  invocation when it detects CI or another known automated environment.
- **AXE.VRS-R17 Baked review prompt:** The VRS semantic review prompt and output
  schema must be owned by VRS/Axe docs and eval-tuned; callers must not provide
  arbitrary prompt files for this command.
- **AXE.VRS-R18 Review-only safety:** `axe vrs review` must invoke the coding
  agent in read-only review mode by default and must not mutate VRS artifacts.
- **AXE.VRS-R19 Fixture review grading:** `axe vrs review-fixtures [path]` must
  run semantic review against an isolated materialized copy of each selected
  semantic-review fixture and grade the returned result against that fixture's
  minimum finding assertions by `rule`, `severity`, `artifact`, and `owner`. It
  must not require matching summary, evidence, or suggested-fix wording, must
  report which assertions were unmet, and must obey the same automated-context
  refusal as `axe vrs review` without adding a token-spend flag.
