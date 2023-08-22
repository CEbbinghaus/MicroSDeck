#!/bin/bash

cargo build --release
mkdir -p ../bin

cp ./target/release/backend ../bin/backend
