#!/bin/bash

RUSTFLAGS=--cfg=web_sys_unstable_apis

cargo build --release --package=markdown-hydrate --target=wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name=module --out-dir=./pkg/blogs/ --target=web ./target/wasm32-unknown-unknown/release/markdown_hydrate.wasm
