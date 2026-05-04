# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`cloud-hypervisor-client` is an unofficial Rust crate wrapping the
[cloud-hypervisor REST API](https://github.com/cloud-hypervisor/cloud-hypervisor/blob/main/docs/api.md).
Cloud-hypervisor exposes its API over a local Unix domain socket, and this crate provides a typed async client
for it.

## Layout

There are two Cargo packages, not configured as a workspace:

- Repo root: the published library crate (`cloud-hypervisor-client`, edition 2021, MSRV 1.81.0).
- `generator/`: a separate, unpublished binary crate (edition 2024) that regenerates the library's API/model code
  from the upstream OpenAPI spec. It is not a path-dependency of the library.

Inside `src/`:

- `src/lib.rs` is **hand-written** and is the only source file in `src/` that survives regeneration. It
  re-exports `apis` and `models` and provides `socket_based_api_client(path) -> SocketBasedApiClient`, which
  wires `hyperlocal::UnixConnector` into a `DefaultApiClient` so callers can target the cloud-hypervisor VMM
  socket without assembling the `Configuration` themselves.
- `src/apis/` and `src/models/` are **auto-generated** by `generator/` from the OpenAPI spec. Do not hand-edit
  these files - they are deleted and rewritten on every regeneration. Persistent changes belong in the generator
  templates instead (see below).

## Common commands

Library (run from repo root):

- `cargo build` / `cargo build --release`
- `cargo test`
- `cargo fmt` / `cargo fmt --check`
- `cargo clippy --all-targets`
- `cargo run --example get_info` (the example expects a real VMM socket at `cloud_hypervisor_vm_socket.sock`
  in CWD and will fail without one).

Code generator:

- `just gen` (from the repo root) is the canonical entry point. It runs the generator binary, which downloads
  the OpenAPI spec and the OpenAPI Generator jar (caches them under `generator/workdir/downloads/`), runs
  codegen, replaces `src/apis` and `src/models`, and runs `cargo fmt` on the library. Requires Java on `PATH`
  (or `JAVA_HOME` set); the jar is fetched from Maven Central and verified against its `.sha1`. Run `just`
  with no arguments to list recipes.
- `cargo run --manifest-path generator/Cargo.toml` works equivalently from any working directory.
  `Dirs::init` anchors `workdir` to the generator crate's `CARGO_MANIFEST_DIR`, so CWD is not significant.

## Code generation pipeline

The generator is `generator/src/main.rs`. The flow:

1. Downloads the upstream spec from the cloud-hypervisor `master` branch and asserts its `info.version` equals
   `cloud_hypervisor_openapi_expected_version` (currently `"0.3.0"` in code). Bumping the upstream spec means
   updating that constant in `main.rs` and verifying the generated diff is sane. Note: the upstream spec has
   been observed to remain pinned at `0.3.0` even after substantive changes (see commit b1c3cb0), so the
   version assertion alone is not sufficient to detect drift. The downloaded spec is written to
   `generator/workdir/downloads/cloud-hypervisor_<version>.yaml` and is **committed** to the repo (the
   generator overwrites it in place on every run via `truncate(true)`); an unexpected diff there after
   `just gen` is the signal that upstream has drifted and the regenerated `src/` should be reviewed against
   it. The OpenAPI Generator jars in the same directory remain gitignored (they are reproducibly fetched
   from Maven Central and SHA1-verified).
2. Runs `openapi-generator-cli` (version `7.22.0`, pinned in `main.rs`) twice:
    - `author template -g rust` into `generator/workdir/templates_original/` to capture the upstream baseline
      templates.
    - `generate` with `--template-dir generator/workdir/templates/` to produce the actual output. The local
      templates are the customized versions that ship in this repo.
3. Deletes `src/apis/` and `src/models/`, moves the generated equivalents in from `generator/workdir/output/`,
   and runs `cargo fmt` on the library.
4. Writes a `git diff -w` between `templates_original/` and `templates/` to `generator/workdir/templates.diff`,
   so reviewers can see what the local template overrides actually change vs. upstream.

`generator/workdir/openapi-generator.yaml` holds the generator config. Key choices:

- `library: hyper` (the generated client uses hyper directly, not reqwest).
- `bestFitInt: true` plus explicit `typeMappings` for `uint8/16/32/64 -> u8/16/32/64`. The `uint*` mapping
  exists because OpenAPI Generator otherwise widens unsigned integers to signed types; commit f05472c added
  this fix and is the kind of change that lives in this YAML.
- `useSingleRequestParameter: true` - operations with multiple parameters take a single struct.
- `avoidBoxedModels: true`.

## Editing rules of thumb

- Library bug fix or hand-written feature -> edit `src/lib.rs` (or non-generated test/example files).
- Output of generated code is wrong (type, naming, structure, derive, etc.) -> edit
  `generator/workdir/templates/` or `generator/workdir/openapi-generator.yaml`, then re-run the generator.
  Do not touch `templates_original/` (it is wiped on every run).
- Spec or generator version bump -> edit the constants near the top of `generator/src/main.rs`.
- After any generator change, run `just gen` and inspect the resulting diff under `src/` before committing.
  Commit the regenerated `src/` together with the template/config change in the same commit (see commit
  b1c3cb0 for the established pattern).

## Things to ignore in `src/apis/` and `src/models/`

The `Configuration` struct still references `reqwest`-style TLS features in doc comments and the README
mentions Cargo features (`default-tls`, `native-tls`, `rustls-tls`) that are not actually declared in
`Cargo.toml`. The runtime client is hyper-only via `hyperlocal::UnixConnector`. Treat those doc fragments
as upstream template artifacts rather than features to wire up.
