build:
	cargo build

run-parse:
	cargo run -- parse data.txt output.json

run-help:
	cargo run -- help

run-credits:
	cargo run -- credits

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
