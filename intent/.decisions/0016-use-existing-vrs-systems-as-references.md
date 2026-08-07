# Use existing VRS systems as references

Status: accepted

## Context

The user asked that the VRS skill mention existing VRS systems as references
when helpful.

## Evidence and Argument

The repo already contains multiple VRS shapes: flat topics, hierarchical
subsystem trees, and composable contract/realization trees. These examples help
agents calibrate scope, section size, subsystem depth, and companion-artifact
use. The risk is cargo-culting a shape that fit another system but not the
current one.

## Options

| Option | Tradeoffs |
| --- | --- |
| Do not mention examples | Avoids copying, but loses useful local precedent. |
| Treat one tree as canonical template | Simple, but too rigid for different system shapes. |
| Use existing systems as references when helpful | Encourages local precedent while preserving fit-to-problem judgment. |

## Decision

VRS authors should inspect existing VRS systems as references when helpful. Use
examples for precedent and calibration, not as templates to copy blindly.
