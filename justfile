
# Build
build:
  cargo build --release

test: build
  cargo test

run: build
  cargo run --release

lint:
  cargo fix
