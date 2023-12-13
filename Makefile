.PHONY: build test clean run update

build:
	cargo build

fmt:
	cargo fmt

test:
	cargo test

clean:
	cargo clean

run:
	cargo run

update:
	cargo update