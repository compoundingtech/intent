# Validate semantic review fixtures deterministically first

Status: accepted

## Context

The first semantic-review fixtures now exist under
`intent/15-evaluation/semantic-review/`, but `intent review` is not yet a
production command. The next question is how to validate those fixtures before a
real LLM-backed review runner exists.

The validator could be a deterministic fixture checker, a CAIC fake-provider
eval, a real-provider eval, or no runner until review lands.

## Evidence and Argument

The fixtures already contain structured machine-readable inputs: `fixture.json`
manifests, expected `axe.intent.review.v1` results, fixture-relative artifact
paths, and references to the baked prompt and schema. Those properties can be
validated without invoking a coding agent.

CAIC fake-provider evals prove invocation wiring but not semantic-review
quality. Real-provider evals provide higher signal about prompt behavior, but
they are slower and more variable than a local structural check. Waiting for
`intent review` would leave the fixture corpus able to drift before it has a
basic contract gate.

## Options

| Option | Tradeoffs |
| --- | --- |
| Deterministic fixture checker | Fast, local, reproducible, and catches fixture/schema/path drift early, but does not prove real review quality. |
| CAIC fake-provider eval | Validates command invocation shape, but does not evaluate the prompt. |
| Real-provider eval | Highest semantic signal, but slower, costlier, and potentially flaky before prompt tuning. |
| No runner yet | Simplest in the short term, but lets fixture drift accumulate. |

## Decision

Validate semantic-review fixtures first with a lightweight deterministic checker
exposed as a Nix check.

The checker validates fixture shape, JSON syntax, prompt/schema references,
expected-review structure, fixture-relative artifact paths, and generated
diagnostic markers when diagnostics are present. It does not call
`$CODING_AGENT` and does not assert prompt quality.

## Consequences

- Fixture drift is caught before semantic review exists.
- Real-provider semantic evals remain a later layer after `intent review` has
  stable diagnostics and invocation wiring.
- The deterministic checker can become a CI/Nix gate without introducing LLM
  flakiness.
