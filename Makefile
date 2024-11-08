format:
    cargo fmt

lint:
    cargo clippy -- -D warnings

run:
    cargo run 

test:
    cargo test

doc:
    cargo doc --open

all: format lint test
