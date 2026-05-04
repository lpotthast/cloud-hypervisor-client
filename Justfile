# Run `cargo install just`. Then run `just` to list available recipes.

default:
  just --list

gen:
  cd ./generator && cargo run
