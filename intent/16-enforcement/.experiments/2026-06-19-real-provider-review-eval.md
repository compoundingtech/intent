# Real provider Intent review eval

The terminology and protocol identifiers in this summary use the current Intent
contract. The original raw provider report used superseded identifiers and was
retired rather than rewritten as captured output.

## Question

After CAIC provider hardening and stock backend-id cleanup, bounded manual
`intent review` runs against a small existing Intent subsystem should complete and
write `axe.intent.review.v1` reports for the supported stock backends.

## Method

Target: `intent/16-enforcement`

Commands used built `axe` and `coding-agent` binaries from the branch, with
`--timeout-seconds 180` and `--report` paths under this experiment directory.
The run was manual and not part of CI or a Nix check.

## Result

| Backend | Result |
| --- | --- |
| `codex` before schema hardening | Failed before semantic output. Improved CAIC diagnostics showed Codex rejected the review schema because `schema_version` used `const` without a `type`. |
| `codex` after adding string types | Failed before semantic output. Codex then rejected the schema because `gate` was a property but not listed in `required`. |
| `codex` after requiring `gate` | Succeeded and produced one semantic-ownership warning. The raw report used superseded identifiers and was retired during the Intent-wide contract migration rather than rewritten as if it were newly captured evidence. |
| `claude` after schema hardening | Failed with CAIC `timeout` after the bounded 180-second run. No report file was written. |
| `claude` wrapper probe after timeout investigation | A tiny direct `--bare` probe spent zero tokens and failed immediately with `Not logged in · Please run /login`, confirming that this worktree has no API-key auth for isolated Claude runs. A CAIC probe then failed before model execution because the wrapper passed retired/unsupported `MultiEdit` in `--disallowedTools`; the installed Claude CLI rejects unknown deny rules. A full `intent review` probe still timed out because CAIC placed variadic `--add-dir <directories...>` immediately before the positional prompt, so Claude consumed the prompt as part of `--add-dir` instead of receiving review input. |
| Fake Claude regression after wrapper fix | Added token-free fake-provider coverage that rejects `MultiEdit`, requires CAIC to pass an explicit `--model sonnet`, and rejects `--add-dir` after `--json-schema` so the prompt cannot be swallowed by Claude's variadic directory option. |

The Codex failures were actionable only after CAIC started returning redacted
provider stdout/stderr tails. A separate tiny direct `codex exec` smoke with the
same core sandbox, `approval_policy=never`, JSON, and output-schema flags
succeeded, confirming that the remaining Codex blocker was the review schema,
not the approval flag.

The successful Codex report found:

```json
{
  "rule": "INTENT.REVIEW.semantic-ownership",
  "severity": "warning",
  "artifact": "intent/16-enforcement/spec.md",
  "owner": "Intent CLI command spec"
}
```

## Conclusion

The deterministic and fake-provider paths are working. Production readiness is
tracked per backend. The stock `codex` real-provider path is now proven for this
small Intent subsystem after schema hardening. The `claude` timeout investigation
found two wrapper-level command
compatibility issues. First, the adapter passed `MultiEdit` in
`--disallowedTools`, but the installed Claude CLI rejects that tool name before
review can proceed. Second, the adapter placed Claude's variadic
`--add-dir <directories...>` option immediately before the positional prompt,
allowing Claude to consume the review prompt as another directory argument.
CAIC now avoids the retired deny rule, places `--add-dir` before later scalar
flags, and pins Claude to `sonnet` instead of the provider default model to
reduce latency and cost variance. A full isolated real-provider Claude report
still requires API-key auth because `--bare` does not use local OAuth/keychain
login. That is now a Claude-backend readiness delta, not a blocker for the
already proven Codex backend.

## Intent Impact

The experiment supports the per-backend readiness rule in the Intent CLI spec and
the remaining Claude-specific delta. It also validates keeping the baked review
schema in `intent/16-enforcement` and keeping real-provider evals
manual-only.
