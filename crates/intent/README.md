# intent

Deterministic checks, graph extraction, and semantic review for a VRS corpus.

The crate ships both a binary (`intent`) and a library. `axe vrs` embeds the
library and calls `intent::run` directly, so the binary is a thin shell over the
same entry point — anything that lived only in `main.rs` would be behavior the
embedded caller silently does not get.

## Commands

| Command                  | What it does                                                     |
| ------------------------ | ---------------------------------------------------------------- |
| `intent check <root>`    | Deterministic VRS checks. `--profile strict\|local`, `--json`.    |
| `intent graph <root>`    | Emits the derived VRS graph subset as JSON.                      |
| `intent review <root>`   | Semantic review via the Coding Agent Invocation Contract.        |
| `intent review-fixtures` | Grades semantic review against evaluation-fixture assertions.    |

## `check` exiting 0 does not mean it read anything

`intent check` on an empty directory, or on a path holding no VRS artifacts at all,
exits **0** with `"diagnostics": []`. `graph` exits 0 there too, with `"nodes": []`.
Neither exit code distinguishes "the corpus is clean" from "the corpus is not
there", so a CI gate built on the exit code alone passes just as happily against a
typo in a path.

What discriminates is the **node count**:

```console
intent check "$corpus" --profile strict --json > report.json
jq -e '(.diagnostics | length) == 0' report.json

intent graph "$corpus" --json > graph.json
jq -e '(.nodes | length) > 0' graph.json   # this is what proves a corpus was read
```

`.github/workflows/ci.yml` is the worked example. Copy the pair, not just the first
half.

## `axe`-flavoured identifiers in an `intent` binary

`check --json` reports `"schema_version": "axe.vrs.check.v1"`, diagnostics are
prefixed `axe vrs check:` / `axe vrs review:`, and the corpus's own requirement ids
are `AXE.VRS-R01..R19`. That is not an oversight.

Note the three are different surfaces, and it matters for anyone planning the rename
below. The `AXE.VRS-R*` ids are **requirement ids in the VRS documents** — this
binary never emits one. The `rule` field of a diagnostic carries a different
vocabulary entirely (`VRS.ENF.link.local-target`, `VRS.ENF.delta-shape`, and four
others). Only `schema_version` and the message prefixes are part of what a consumer
parses.

This crate was lifted out of `schickling/dotfiles`' `axe` binary, and the acceptance
bar for the lift is that behaviour is byte-identical to `axe vrs` across every
command. Renaming these strings now would destroy the differential oracle that
proves the extraction was faithful, and `axe.vrs.check.v1` is a wire contract with a
live consumer. They are carried to one coordinated rename pass — rule ids,
`schema_version`, the schema `$id` host and the message prefixes together, never
piecemeal.

## Enforcement assets resolve under the corpus, never the repository

`review` reads two assets from the filesystem at runtime:

```
<corpus>/16-enforcement/review-prompt.md
<corpus>/16-enforcement/review-result.schema.json
```

Both are resolved **relative to the corpus root** given on the command line, not
relative to the enclosing repository. A corpus that moves takes its enforcement
assets with it, which is what keeps tool and corpus co-located.

The consequence is deliberate and is a behavior change from earlier versions: a
corpus that has no `16-enforcement/` of its own **fails** rather than quietly
borrowing the enclosing repository's copies. Silently falling back produced a
review graded against a rubric the corpus never declared — passing for reasons
its own contents could not account for.

The failure is exit code `2` on stderr, and it names both the asset that is
missing and the corpus it was missing from:

```console
$ intent review ./some-corpus
axe vrs review: missing review asset 16-enforcement/review-prompt.md under corpus /abs/path/to/some-corpus
```

Naming both matters: the asset alone does not say which of several corpora was
searched, and the root alone does not say what it was expected to contain.

`check` and `graph` do not read these assets and are unaffected.

## Layout note

`crates/intent` is a standalone package — it has its own `Cargo.lock` and there
is no workspace root above it. Cargo therefore writes build output to
`crates/intent/target/`, not to `./target/`. Anything invoking the built binary
by path from the repository root should pass `--target-dir` explicitly rather
than assume either location; `.github/workflows/ci.yml` does exactly that.

## Development

```console
cargo build --locked --manifest-path crates/intent/Cargo.toml
cargo test  --locked --manifest-path crates/intent/Cargo.toml
```

The repository is also a flake, which is the only supported distribution — there
is no crates.io release. `nix build .#intent` packages the CLI, `nix flake check`
runs fmt, clippy, the test suite and a proof that the packaged binary reads a
real corpus, and `nix develop` gives you the toolchain plus `jq` and
`check-jsonschema` that the corpus gates use.

`rust-toolchain.toml` sits at `crates/intent/`, and it is in effect in fewer places
than it looks. rustup resolves it from the **working directory** upward, not from
`--manifest-path` — so a command run at the repository root never sees it, which
includes every CI job here. A Nix build does not read it either; it uses whichever
toolchain nixpkgs pins. In practice it applies when you are working inside
`crates/intent/` with rustup, and nowhere else. That is fine, because the crate
pins no MSRV — but the three toolchains are not expected to agree on a patch
version, so do not read agreement into a green run.
