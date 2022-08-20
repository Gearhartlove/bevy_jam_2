#!/usr/bin/env sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bevy_jam_2.wasm
now=$(date +'%m%d%Y')

rm -r ~/Desktop/bevybuild$now

mkdir ~/Desktop/bevybuild$now
cp -r ./assets ~/Desktop/bevybuild$now
cp -r ./web ~/Desktop/bevybuild$now
cp ./index.html ~/Desktop/bevybuild$now
