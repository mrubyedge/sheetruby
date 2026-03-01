.PHONY: build clean

build:
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(EMSDK)/upstream/emscripten/cache/sysroot" \
		cargo build --target wasm32-unknown-emscripten --release
	cat pre.js out.js main.js > combined.js

clean:
	cargo clean
	rm -f combined.js out.js
