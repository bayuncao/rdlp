.PHONY: build test clean run update

build:
	cargo build

test:
	cargo test

clean:
	cargo clean

run:
	cargo run

update:
	cargo update