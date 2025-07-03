#!/bin/sh
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
exec trunk serve --open
