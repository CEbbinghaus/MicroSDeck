#!/bin/bash

cargo build --release --features ,$1
mkdir -p ../bin

cp ./target/release/backend ../bin/backend
