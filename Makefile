build:
	cargo wasm

test-unit:
	RUST_BACKTRACE=1 cargo unit-test
