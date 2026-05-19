## Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0+api-spec-0.3.0-2026-05-19] - 2026-05-19

### cloud-hypervisor-client

#### Added

- **Breaking.** New optional fields on `models::ConsoleConfig`:
    - `id: Option<String>`
    - `pci_segment: Option<i16>`
    - `pci_device_id: Option<u8>`
- New `models::SerialConfig` (fields `file: Option<String>`, `socket: Option<String>`, `mode: models::ConsoleMode`).

#### Changed

- **Breaking.** `VmConfig::serial` is now `Option<models::SerialConfig>` instead of `Option<models::ConsoleConfig>`.
- **Breaking.** `PlatformConfig::iommu_address_width` renamed to `iommu_address_width_bits`.

## [0.5.0+api-spec-0.3.0-2026-05-11] - 2026-05-11

### cloud-hypervisor-client

#### Added

- **Breaking.** New optional fields on `models::RngConfig`:
    - `id: Option<String>`
    - `pci_segment: Option<i16>`
    - `pci_device_id: Option<u8>`
- **Breaking.** New optional field on `models::DeviceConfig`: `x_exclude_mmap_bars: Option<Vec<i64>>`.

#### Changed

- **Breaking.** Integer fields whose upstream `format` is one of `uint8` / `uint16` / `int8` /
  `int16` now render as the matching Rust integer type instead of the `i32` fallback OpenAPI
  Generator previously produced. Driven by the new generator-side normalization pass (see below).
  The complete set of affected fields:
    - `pci_segment` on every `*Config` model that carries one: `i32` -> `i16`, or `Option`-wrapped where required.
    - `pci_device_id` on every `*Config` model that carries one: `i32` -> `u8`, or `Option`-wrapped where required.
    - `DeviceConfig::x_nv_gpudirect_clique`: `Option<i32>` -> `Option<i8>`.
    - `PlatformConfig::num_pci_segments`: `Option<i32>` -> `Option<i16>`.
    - `PlatformConfig::iommu_segments`: `Option<Vec<i32>>` -> `Option<Vec<i16>>`.
    - `PlatformConfig::iommu_address_width`: `Option<i32>` -> `Option<u8>`.
- **Breaking.** Field declaration order inside `RngConfig` changed. `src` moves from first to last.

### generator

#### Added

- `generator/src/normalize.rs`: post-fetch pass that rewrites every `{ type: integer, format: <int-format> }` schema in
  the downloaded spec to `{ type: <int-format> }`. `openapi-generator.yaml`'s `typeMappings:` block then resolves every
  integer format to its Rust counterpart, replacing OpenAPI Generator's silent `i32` widening for every format besides
  `int32` / `int64`.
- `generator/workdir/downloads/cloud-hypervisor_<version>.normalized.yaml`: the post-rewrite spec fed to OpenAPI
  Generator.
- `generator/workdir/downloads/cloud-hypervisor_<version>.normalized.diff`: unified diff between the original and
  normalized YAML files. Generated and committed for review.

#### Changed

- `openapi-generator.yaml` `typeMappings:` gained `int8` / `int16` / `int32` / `int64` entries to match the existing
  `uint8` / `uint16` / `uint32` / `uint64` ones.

## [0.4.0+api-spec-0.3.0-2026-05-04] - 2026-05-04

### cloud-hypervisor-client

#### Added

- Two new generated API operations on `DefaultApi`, regenerated from a newer revision of the upstream cloud-hypervisor
  OpenAPI spec (still self-labelled `0.3.0`):
    - `vm_add_generic_vhost_user_put` (`PUT /vm.add-generic-vhost-user`)
    - `vm_resize_disk_put` (`PUT /vm.resize-disk`)
- New generated models: `ConsoleMode`, `CoreSchedulingMode`, `GenericVhostUserConfig`, `ImageType`, `LockGranularity`,
  `MemoryRestoreMode`, `TimeoutStrategy`, `UserDeviceConfig`, `VmResizeDisk`, `VmState`.
- Various new optional fields on existing `*Config` models from the same regeneration pass; see commit `b1c3cb0` for
  the full set.

#### Changed

- **Breaking.** Bumped MSRV from `1.81.0` to `1.86.0` and migrated the library crate to Rust edition 2024.
- **Breaking.** Dropped HTTP authentication from the public surface. `apis::configuration::Configuration` no
  longer exposes `basic_auth`, `oauth_access_token`, or `api_key`, and the `BasicAuth` alias and `ApiKey` struct
  are gone. The cloud-hypervisor REST API speaks over a Unix domain socket and defines no `securitySchemes`, so
  these fields had no effect against cloud-hypervisor itself. Callers that constructed `Configuration` as a struct
  literal must drop the auth initializers. Anyone tunneling through an authenticating proxy can attach headers on
  the `hyper::Client` passed to `Configuration::with_client`.
- Bumped the `http` dependency from `~0.2` to `1`. The library only references `http::header::InvalidHeaderValue`,
  `http::Error`, and `http::uri::InvalidUri` (in `apis::Error` variants), all of which exist with identical shape
  in `http 1.x`. Aligns the dep tree with the `http 1.x` already pulled in transitively by `hyper 1.x`.
- `ConsoleConfig::mode` and `DebugConsoleConfig::mode` now use the shared `models::ConsoleMode` enum instead of
  per-struct inline `Mode` enums. Callers constructing these configs need to switch to `models::ConsoleMode::*`.
- `VmInfo::state` now uses `models::VmState` instead of an inline per-struct `State` enum.

#### Removed

- `models::SgxEpcConfig` (dropped from the upstream spec).
- `base64` runtime dependency. Its only call site was `base64::encode` in the `Auth::Basic` arm of `request.rs`,
  which the auth removal above eliminates.

### generator

#### Added

- Generator config maps unsigned integer schema types (`uint8` / `uint16` / `uint32` / `uint64`) to their Rust
  counterparts (`u8` / `u16` / `u32` / `u64`) instead of the signed types OpenAPI Generator otherwise picks (#3).
- Generator performs a pre-flight check against Maven Central and emits a `tracing::warn!` when a newer
  `openapi-generator-cli` release is available than the pinned one.
- Top-level `Justfile` with a `just gen` recipe that drives the code generator from the repo root.
- templates/hyper/configuration.mustache override, driving the auth removal from generated code.

#### Changed

- Generator binary (`generator/src/main.rs`) anchors its `workdir` to `CARGO_MANIFEST_DIR` rather than the current
  working directory, so it can be invoked from anywhere (e.g. `cargo run --manifest-path generator/Cargo.toml` from the
  repo root).
- The generator source is now spread across multiple modules with main.rs only acting as a thin orchestrator.
- Constants for the generator/spec versions and URLs are now lifted to the top of module file.
- Bumped the OpenAPI Generator CLI pinned by the generator from `7.12.0` to `7.22.0`. Regenerated the
  `templates_original/` baseline against 7.22.0; generated `src/` is byte-identical for the current spec.
- Generator dependencies updated, including new entries for `quick-xml`, `semver`, and `serde` (used by the new
  release check).
- Switched the error-handling library from `anyhow` to `rootcause` (with the `rootcause-backtrace` and
  `rootcause-tracing` companions). `main()` now installs both hooks at startup so error reports automatically
  capture a backtrace and the active tracing spans, and `tracing_init` adds `RootcauseLayer` to the subscriber so
  span fields are recorded.

#### Removed

- Runtime check that required the git checkout to literally be named `cloud-hypervisor-client`.
- Local `generator/workdir/templates/hyper/api.mustache` and `generator/workdir/templates/model.mustache`. They had
  no intentional overrides and were stale 7.12.0-era copies; the generator now uses the embedded 7.22.0 versions via
  OpenAPI Generator's `--template-dir` fallback.

## [0.3.3] - 2025-06-20

### cloud-hypervisor-client

#### Added

- `apis::Error` now implements `std::error::Error` and `Display`, courtesy of `thiserror` annotations baked into the
  local `hyper/api_mod.mustache` template (PR #1). Each variant has a stable `Display` formatting.
- New runtime dependency: `thiserror` 2.x.
- Declared MSRV: Rust 1.81.0, recorded as `rust-version` in `Cargo.toml` (PR #2).

## [0.3.2] - 2025-06-16

### cloud-hypervisor-client

#### Changed

- README cleanup: dropped the redundant "for Rust" suffix from the heading.

#### Removed

- Unused runtime dependencies: `serde_with` (and its `base64` / `std` / `macros` features) and `rand` (dev dependency).
  These were artifacts of earlier generator output and never used by the published code.

### generator

#### Changed

- Bumped the OpenAPI Generator CLI version pinned by the generator to `7.12.0`.
- Updated the local `model.mustache` template to track upstream changes shipped in OpenAPI Generator 7.12.0.

## [0.3.1] - 2025-06-12

### cloud-hypervisor-client

#### Added

- Generated docstrings on `NetConfig::ip` and `NetConfig::mask` describing the accepted IPv4 / IPv6 formats, picked up
  from a regenerated (non-breaking) pass over the spec.

### generator

#### Changed

- Generator crate switched to Rust edition 2024 and explicitly marked `publish = false` so it can never be uploaded to
  crates.io alongside the library.
- Generator dependencies refreshed (minor / patch bumps): `anyhow`, `assertr`, `futures-util`, `reqwest`, `tokio`,
  `tracing`, `tracing-subscriber`, `yaml-rust2`.

## [0.3.0] - 2025-04-11

### cloud-hypervisor-client

#### Added

- Initial public release on crates.io.
- Hyper-based async client for the cloud-hypervisor REST API, generated from upstream OpenAPI spec
  `info.version = 0.3.0`. The generated client is exposed via `apis::DefaultApi` / `apis::default_api::DefaultApiClient`
  with all request/response types under `models::`.
- `socket_based_api_client(path) -> SocketBasedApiClient` helper in `lib.rs` that wires `hyperlocal::UnixConnector`
  into a `Configuration` so callers can talk to the VMM Unix-domain socket without assembling hyper plumbing.
- API errors are surfaced as `String`s rather than raw `hyper::body::Incoming` bodies (template override).
- `cargo run --example get_info` example demonstrating a basic VM-info call against a local VMM socket.

### generator

#### Added

- Companion `generator/` binary crate that downloads the upstream spec, fetches and SHA1-verifies the OpenAPI Generator
  jar (7.9.0 at this release) from Maven Central, runs codegen with custom mustache templates, and rewrites `src/apis`
  / `src/models`. Generator config sets `library: hyper`, `avoidBoxedModels: true`, `bestFitInt: true`, and
  `useSingleRequestParameter: true`.

[Unreleased]: https://github.com/lpotthast/cloud-hypervisor-client/compare/v0.6.0+api-spec-0.3.0-2026-05-19...HEAD

[0.6.0+api-spec-0.3.0-2026-05-19]: https://github.com/lpotthast/cloud-hypervisor-client/compare/v0.5.0+api-spec-0.3.0-2026-05-11...v0.6.0+api-spec-0.3.0-2026-05-19

[0.5.0+api-spec-0.3.0-2026-05-11]: https://github.com/lpotthast/cloud-hypervisor-client/compare/v0.4.0+api-spec-0.3.0-2026-05-04...v0.5.0+api-spec-0.3.0-2026-05-11

[0.4.0+api-spec-0.3.0-2026-05-04]: https://github.com/lpotthast/cloud-hypervisor-client/compare/v0.3.3...v0.4.0+api-spec-0.3.0-2026-05-04

[0.3.3]: https://github.com/lpotthast/cloud-hypervisor-client/compare/v0.3.2...v0.3.3

[0.3.2]: https://github.com/lpotthast/cloud-hypervisor-client/compare/v0.3.1...v0.3.2

[0.3.1]: https://github.com/lpotthast/cloud-hypervisor-client/compare/v0.3.0...v0.3.1

[0.3.0]: https://github.com/lpotthast/cloud-hypervisor-client/releases/tag/v0.3.0
