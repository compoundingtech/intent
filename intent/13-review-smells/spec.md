# VRS Review Smells — Spec

This document specifies VRS review smells. It builds on
[requirements.md](./requirements.md).

## Status

Draft.

## Smell Catalog

| Smell | Fix |
| --- | --- |
| `vision.md` names a tool, architecture, implementation strategy, or migration plan. | Move mechanism to `requirements.md`, `spec.md`, or `.decisions/`. |
| `vision.md` forces every topic into "The Problem" even when need/opportunity/domain tension is clearer. | Use the framing that keeps the vision non-technical and durable. |
| `requirements.md` says how to build the system. | Move mechanism to `spec.md`; keep requirements testable. |
| A non-negotiable platform/API/resource/compliance limit appears as a normal requirement. | Move it to `## Constraints` and cite `.reference/` when external. |
| `spec.md` explains why an unusual choice was made. | Promote the rationale to a decision record and keep spec operational. |
| `ontology.md` contains behavior, API shape, or decision history. | Move behavior to spec and rationale to decisions. |
| `open-questions.md` contains resolved answers. | Move the answer to its owner and delete the question. |
| `roadmap.md` reads like current contract. | Promote it to requirements/spec/decision or weaken it to non-normative future direction. |
| `.delta/` contains stale, vague, speculative, duplicate, or completed entries. | Update or delete the delta. |
| `.decisions/.proposed/` would merge to main. | Accept, fold, defer to open questions, or delete proposed records before merge. |
| A decision embeds raw experiment logs, benchmark tables, transcripts, or follow-up backlog. | Summarize the decisive evidence and link to `.experiments/`, `.reference/`, code, planning, or the owning VRS artifact. |
| `.experiments/` contains plans instead of evidence. | Move plans to the planning system; keep only evidence records. |
| `.experiments/` aggregates unrelated questions until the conclusion is hard to promote. | Split by major question, hypothesis, or validation method. |
| `.reference/` contains copied external docs without VRS impact. | Summarize relevant facts and link to the source. |
| Empty companion directories exist to satisfy a template. | Delete them until real content exists. |
| A generic `README.md` is used as VRS entry point. | Use `intuition.md`; keep README for repository onboarding only. |
| Requirements exceed 30 items or approach 40. | Split into child VRS nodes. |

Agents should run this catalog before finalizing VRS edits and fix any smell in
the same change when the correct owner is clear.
