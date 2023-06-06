FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:latest

RUN pacman -S --noconfirm cmake make clang git

# Updates the Crates.io index
RUN cargo search --limit 0

ENTRYPOINT [ "/backend/entrypoint.sh" ]
