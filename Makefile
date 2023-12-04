test:
	RUST_LOG=debug cargo test

build:
	cargo build -r

dev:
	RUST_LOG=debug cargo run

run:
	RUST_LOG=info ./target/release/advent
