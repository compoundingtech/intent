# Do not require an extra token-spend flag for review

Status: accepted

## Context

Decision 0028 established that real provider/model-token VRS evals are
manual-only and must not run from CI, Nix checks, hooks, scheduled jobs, or
default automated validation.

The remaining question is whether `axe vrs review` also needs an extra
invocation flag such as `--manual`, `--spend-tokens`, or `--allow-token-spend`
to prove consent before calling a real provider.

## Evidence and Argument

`axe vrs review` is itself an explicit operator command. Requiring an additional
flag makes the command more ceremonious without adding a stronger automation
boundary. The actual safety property is that automated environments must not
invoke a real provider. That is enforced by refusing CI and other known
automation indicators before provider invocation.

Additional token-spend flags can also create a false sense of safety: scripts
can pass flags too. CI refusal and careful non-wiring into automatic checks are
the principled safeguards. Documentation should make the cost model clear, but
the command surface should stay simple.

## Options

| Option | Tradeoffs |
| --- | --- |
| No extra flag; refuse automation | Keeps the operator command simple and makes the real guard CI/automation refusal, but relies on clear docs that review may spend tokens. |
| Require `--manual` | Easy to test, but vague and redundant once the command is explicitly named `review`. |
| Require `--spend-tokens` or `--allow-token-spend` | Names the cost risk, but adds ceremony and can still be passed by scripts. |
| Environment override | Script-friendly, but too easy to leak into automated environments. |

## Decision

`axe vrs review` does not require an extra token-spend or manual-confirmation
flag.

The command must fail before provider invocation when it detects CI or another
known automated environment. It must not be wired into CI, Nix checks, hooks,
scheduled jobs, or default automation.

## Consequences

- The manual review command remains simple to run intentionally.
- The automation boundary is enforced by environment refusal and by not wiring
  real-provider review into automatic gates.
- Fake-provider tests remain appropriate for automated coverage.
