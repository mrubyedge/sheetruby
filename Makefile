.PHONY: build clean

RUSTUP_TOOLCHAIN ?= nightly-2026-06-01

build:
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(EMSDK)/upstream/emscripten/cache/sysroot" \
		cargo +$(RUSTUP_TOOLCHAIN) build --target wasm32-unknown-emscripten --release
	cat pre.js out.js main.js > combined.js

clean:
	cargo clean
	rm -f combined.js out.js
