FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:latest

RUN pacman -S --noconfirm cmake make clang git base-devel musl

RUN rustup target add x86_64-unknown-linux-musl

RUN rustup --version && cargo --version && rustc --version

ENTRYPOINT [ "/backend/entrypoint.sh" ]
