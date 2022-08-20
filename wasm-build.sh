#!/usr/bin/env sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web/ --target web ./target/wasm32-unknown-unknown/release/bevy_jam_2.wasm
# get current date
now=$(date +'%m%d%Y')
rm -r ~/Desktop/bevy_jam_2_builds/bevybuild$now

mkdir ~/Desktop/bevy_jam_2_builds/bevybuild$now
cp -r ./assets ~/Desktop/bevy_jam_2_builds/bevybuild$now
cp -r ./web ~/Desktop/bevy_jam_2_builds/bevybuild$now
cp ./index.html ~/Desktop/bevy_jam_2_builds/bevybuild$now
