#!/bin/bash
rm -rf wasm;
rm -rf wasm.zip;
cargo build --release --target wasm32-unknown-unknown;
wasm-bindgen --no-typescript --out-name bevy_game --out-dir wasm --target web target/wasm32-unknown-unknown/release/bag_goblin.wasm;
cp -r assets wasm/;
cp index.html wasm/;
