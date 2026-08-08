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

Note that `rust-toolchain.toml` pins the channel for rustup users only. A Nix
build uses whichever toolchain nixpkgs pins and does not read that file; the two
are not expected to agree on a patch version.
