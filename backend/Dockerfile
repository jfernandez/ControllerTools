FROM ghcr.io/steamdeckhomebrew/holo-toolchain-rust:latest

RUN pacman -Sy --noconfirm pkgconf

ENTRYPOINT [ "/backend/entrypoint.sh" ]