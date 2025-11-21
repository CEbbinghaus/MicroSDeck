#!/bin/sh
set -e

echo "Container's IP address: `awk 'END{print $1}' /etc/hosts`"

rustup default stable
rustup target add x86_64-unknown-linux-musl

cd /backend

sudo bash build-docker.sh
