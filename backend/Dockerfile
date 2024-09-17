FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:d071cc098dd45b7ab3564ab91b13e809f7020bd6bf8e9166cfaef55401ccd776

RUN pacman -S --noconfirm cmake make clang git

ENTRYPOINT [ "/backend/entrypoint.sh" ]
