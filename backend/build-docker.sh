#!/bin/bash

echo "--- Rust version info ---"
rustup --version
rustc --version
cargo --version

mkdir -p out

echo "--- Building plugin backend ---"
cargo build --profile docker
BUILD_EXIT=$?
mkdir -p out

mv target/docker/backend out/backend

echo " --- Cleaning up ---"
# remove root-owned target folder
cargo clean
# remove newly-cloned git repo and artifacts
rm -rf ./ryzenadj
exit $BUILD_EXIT