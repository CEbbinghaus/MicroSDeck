FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:latest

RUN pacman -S --noconfirm cmake make clang git base-devel musl

RUN rustup target add x86_64-unknown-linux-musl

ARG CACHEBUST=1

RUN rustup --version && cargo --version && rustc --version && git --version

RUN rustup default stable

ENTRYPOINT [ "/backend/entrypoint.sh" ]
