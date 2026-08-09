{
  description = "intent - deterministic checks, graph extraction and semantic review for an Intent corpus";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # The crate is at `crates/intent`, not the repository root: it is a
        # standalone package with no workspace above it. Every path below that
        # reaches into the source has to say so, which is why the manifest, the
        # lockfile and `buildAndTestSubdir` are all spelled out rather than
        # defaulting to `./`.
        crateDir = "crates/intent";

        # Cargo.toml is the single source of truth for the version, so a release
        # bump needs no matching edit here.
        version = (builtins.fromTOML (builtins.readFile ./crates/intent/Cargo.toml)).package.version;

        intent = pkgs.rustPlatform.buildRustPackage {
          pname = "intent";
          inherit version;
          src = self;

          # No git or crates.io-yanked deps in the lockfile, so the lockfile alone
          # pins every input reproducibly — no per-dep outputHashes, and nothing
          # here to hand-patch when a dep bumps.
          cargoLock.lockFile = ./crates/intent/Cargo.lock;

          # Builds and tests run inside the crate, while `src` stays the whole
          # repository. That is deliberate: the corpus at `intent/` has to remain
          # visible so a check can be aimed at it from the same source tree.
          buildAndTestSubdir = crateDir;

          # Both are required, and they are not the same knob.
          # `buildAndTestSubdir` only moves the build and test phases;
          # `cargoSetupPostPatchHook` still reconciles the vendored lockfile
          # against `$sourceRoot/Cargo.lock` — the REPOSITORY root — and fails
          # with "Missing Cargo.lock from src" because none is there. `cargoRoot`
          # is what points that reconciliation at the crate.
          cargoRoot = crateDir;

          # `rust-toolchain.toml` pins the channel for rustup users. A Nix build
          # deliberately does not honour it — the toolchain here is whichever one
          # nixpkgs pins, which is the point of building this way. The file stays
          # for native development; the two are not expected to agree on a patch
          # version.

          meta = {
            description = "Deterministic checks, graph extraction and semantic review for an Intent corpus";
            homepage = "https://github.com/compoundingtech/intent";
            mainProgram = "intent";
          };
        };
      in
      {
        packages.intent = intent;
        packages.default = intent;

        # These gate the CRATE only. The corpus gates deliberately live as their
        # own jobs in `.github/workflows/ci.yml` and are NOT mirrored here: folding
        # them behind `nix flake check` would collapse `corpus-strict` and
        # `semantic-review-fixtures` into a single check named `check`, and a run
        # would no longer show which of the two concluded and how. This lane is
        # additive — it packages the CLI, it does not re-gate the corpus.
        checks.intent = intent;

        checks.fmt =
          pkgs.runCommand "intent-fmt-${version}"
            {
              nativeBuildInputs = [
                pkgs.cargo
                pkgs.rustfmt
              ];
            }
            ''
              cd ${self}/${crateDir}
              cargo fmt --check --manifest-path Cargo.toml
              touch $out
            '';

        checks.clippy = intent.overrideAttrs (old: {
          pname = "intent-clippy";
          nativeBuildInputs = (old.nativeBuildInputs or [ ]) ++ [ pkgs.clippy ];
          # A custom `buildPhase` replaces `cargoBuildHook`, which is what
          # `buildAndTestSubdir` acts through — so without this `pushd` cargo runs
          # at the repository root and dies on "could not find `Cargo.toml`".
          # Vendoring is set up under the crate too (see `cargoRoot`), so the
          # working directory has to be the crate either way.
          buildPhase = ''
            runHook preBuild
            pushd ${crateDir}
            cargo clippy --locked --all-targets -- -D warnings
            popd
            runHook postBuild
          '';
          # Nothing to test or install: the lint IS the result. `touch $out` keeps
          # the derivation honest about producing an output.
          doCheck = false;
          installPhase = "touch $out";
        });

        # Smoke test that the built binary actually runs and its command tree is
        # wired, independent of the in-tree `cargo test`.
        checks.help = pkgs.runCommand "intent-help-${version}" { } ''
          ${intent}/bin/intent --help > /dev/null
          ${intent}/bin/intent check --help > /dev/null
          ${intent}/bin/intent graph --help > /dev/null
          ${intent}/bin/intent review --help > /dev/null
          touch $out
        '';

        # Proves the PACKAGED binary reads a real corpus, not just that it builds.
        # `check` alone cannot carry this: it exits 0 on an empty directory, so a
        # passing check is consistent with having read nothing. The graph is what
        # discriminates — empty for both an empty directory and a wrong path.
        checks.reads-the-corpus = pkgs.runCommand "intent-reads-the-corpus-${version}" {
          nativeBuildInputs = [ pkgs.jq ];
        } ''
          ${intent}/bin/intent graph ${self}/intent --json > graph.json
          nodes="$(jq '.nodes | length' graph.json)"
          echo "graph nodes: $nodes"
          jq -e '(.nodes | length) > 0' graph.json > /dev/null \
            || { echo "packaged binary examined 0 artifacts" >&2; exit 1; }
          touch $out
        '';

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.clippy
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.jq
            pkgs.check-jsonschema
          ];
        };
      }
    );
}
