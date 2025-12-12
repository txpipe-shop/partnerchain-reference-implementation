# Build stage
FROM rust:1.90-bullseye AS builder

RUN apt-get update && \
    apt-get install -y \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    protobuf-compiler \
    --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

# Configure Rust toolchain
RUN rustup default 1.90 && \
    rustup update && \
    rustup target add wasm32v1-none --toolchain 1.90 && \
    rustup component add rust-src --toolchain 1.90

# Create and set working directory
WORKDIR /reference-impl

# Copy only necessary project files
COPY Cargo.toml Cargo.lock ./
COPY node/ node/
COPY runtime/ runtime/
COPY griffin-core/ griffin-core/
COPY griffin-rpc/ griffin-rpc/
COPY demo/ demo/
COPY wallet/ wallet/
COPY game/ game/
COPY toolkit/ toolkit/

# Build the node
RUN cargo build --release -p gpc-node
RUN cargo build --release -p gpc-wallet

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies and curl
RUN apt-get update && \
apt-get install -y \
curl \
--no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /reference-impl/target/release/gpc-node /usr/local/bin
COPY --from=builder /reference-impl/target/release/gpc-wallet /usr/local/bin
COPY docker/genesis.json .
COPY docker/examples/ /examples

# Create directory for chain data
RUN mkdir -p /data