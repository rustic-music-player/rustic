#!/usr/bin/env bash
rm -rf clients/http/wasm/pkg
wasm-pack build clients/http/wasm --out-name rustic-http-client
cp clients/http/wasm/package.json clients/http/wasm/pkg/
wasm-pack pack clients/http/wasm
