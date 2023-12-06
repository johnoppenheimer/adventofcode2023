test:
	RUST_LOG=debug cargo test

build:
	cargo build -r

dev:
	RUST_LOG=debug cargo run

run:
	RUST_LOG=info ./target/release/advent

flame: export RUST_LOG=info
flame: export CARGO_PROFILE_RELEASE_DEBUG=true
flame:
	cargo flamegraph --bin=advent --flamechart --root
