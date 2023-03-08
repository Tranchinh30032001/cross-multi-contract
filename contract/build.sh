#!/bin/sh

echo ">> Building contract"

rustup target add wasm32-unknown-unknown
RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release
