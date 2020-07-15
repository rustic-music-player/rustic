CC = gcc
CCFLAGS = -B target/debug -iquote . -lrustic_ffi_client

wasm:
	./clients/http/wasm/package.sh

wasm-publish: wasm
	cd clients/http/wasm/pkg; npm publish

ffi: ffi-header ffi-library

ffi-library:
	cargo build -p rustic-ffi-client

ffi-header:
	cargo +nightly expand -p rustic-ffi-client > ffi-client.rs
	cbindgen -o bindings.h -c clients/ffi/cbindgen.toml ffi-client.rs
	rm ffi-client.rs

ffi-examples: target/ffi/sync_http_interop target/ffi/cb_http_interop ffi
	LD_LIBRARY_PATH=target/debug ./target/ffi/sync_http_interop
	LD_LIBRARY_PATH=target/debug ./target/ffi/cb_http_interop

target/ffi/sync_http_interop: clients/ffi/tests/sync_http_interop.c
	mkdir -p target/ffi
	$(CC) $(CCFLAGS) -o target/ffi/sync_http_interop clients/ffi/tests/sync_http_interop.c

target/ffi/cb_http_interop: clients/ffi/tests/cb_http_interop.c
	mkdir -p target/ffi
	$(CC) $(CCFLAGS) -o target/ffi/cb_http_interop clients/ffi/tests/cb_http_interop.c

extensions: uwu party-mode

uwu:
	cargo build -p rustic-uwu-extension

party-mode:
	cargo build -p rustic-party-mode-extension
