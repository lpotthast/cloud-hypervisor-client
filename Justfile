# Run `cargo install just`. Then run `just` to list available recipes.

default:
  just --list

# Install dependencies for maintenance work, profiling and more...
install-tools:
  cargo +stable install --locked cargo-minimal-versions
  cargo +stable install --locked cargo-msrv

# Generate the library source using our `generator/`.
gen:
  cd ./generator && cargo run

# Run everything that CI runs, locally (skipping the MSRV check).
verify:
  cargo fmt --all -- --check
  cargo check --all-targets --locked
  cargo test --all-targets --locked
  cargo build --release --all-targets --locked
  cargo doc --release --locked --no-deps
  cargo check --manifest-path generator/Cargo.toml --all-targets --locked
  cargo clippy --manifest-path generator/Cargo.toml --all-targets --locked -- -D warnings
  cargo test --manifest-path generator/Cargo.toml --all-targets --locked

# Find the minimum supported rust version.
msrv:
    cargo msrv find

# Check if the current dependency version bounds are sufficient.
minimal-versions:
    cargo minimal-versions check --workspace --direct
