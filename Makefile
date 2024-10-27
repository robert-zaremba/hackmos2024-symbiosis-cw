build:
	@cargo wasm
	@mkdir -p build
	mv target/wasm32-unknown-unknown/release/hackmos_affiliate.wasm ./build/

.PHONY: build

test-unit:
	RUST_BACKTRACE=1 cargo unit-test
