build:
	cargo wasm

test-unit:
	RUST_BACKTRACE=1 cargo unit-test

optimize:
	@docker run --rm -v "$$(pwd)":/code \
  --mount type=volume,source="$$(basename "$$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.16.1


