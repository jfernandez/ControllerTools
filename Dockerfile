FROM ubuntu:22.04 AS builder

RUN apt-get update && \
    apt-get install -y build-essential git curl npm zip pkg-config libudev-dev jq

# Version supported by the Decky CI
RUN npm install -g pnpm@6.0.0

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Configure environment for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY . .

RUN pnpm install --force

RUN ./release.sh

FROM scratch AS binaries

COPY --from=builder /app/pnpm-lock.yaml /
COPY --from=builder /app/*.zip /