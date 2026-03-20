FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:latest

RUN pacman -S --noconfirm cmake make clang git base-devel musl

RUN rustup target add x86_64-unknown-linux-musl

ARG USER=devcontainer
RUN groupadd -g 100 default && \
    useradd -m -u 1000 -g default $USER && \
	echo "$USER ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \
	chown $USER /home/$USER

RUN curl https://mise.run | sudo -u $USER sh && \
	mkdir /home/$USER/.local/share && \
	echo 'eval "$(~/.local/bin/mise activate bash)"' > /home/$USER/.bashrc

ENTRYPOINT [ "/backend/entrypoint.sh" ]
