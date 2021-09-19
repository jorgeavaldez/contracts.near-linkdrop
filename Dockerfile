FROM rust:1.51.0

WORKDIR /app

LABEL description="Container for contract builds"

RUN rustup default 1.51.0
RUN rustup target add wasm32-unknown-unknown

RUN apt-get update && apt-get install -y \
    git \
    less \
    vim \
    llvm \
    clang \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY near-campaign ./near-campaign/
COPY user ./user/
COPY linkdrop ./linkdrop/