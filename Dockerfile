# syntax=docker/dockerfile:1.6
# ---- Chef stage (Base environment) ----
# Pinned to rust:1.93 digest to ensure cache stability
FROM rust@sha256:ecbe59a8408895edd02d9ef422504b8501dd9fa1526de27a45b73406d734d659 AS chef
RUN cargo install --locked cargo-chef --version 0.1.68
WORKDIR /app

# Install cross-compilation tools for aarch64 (Cached layer)
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu libc6-dev-arm64-cross clang libclang-dev

# Add the target (Cached layer)
RUN rustup target add aarch64-unknown-linux-gnu

# Configure environment for cross-compilation
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
    SQLX_OFFLINE=true

# ---- Planner stage ----
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- Builder stage ----
FROM chef AS builder

# Cook dependencies (This is the critical cached layer!)
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --target aarch64-unknown-linux-gnu --recipe-path recipe.json --package skyclaw

# Pre-install nab binary for deployment (Cached unless version changes)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    ~/.cargo/bin/cargo-binstall -y --target aarch64-unknown-linux-gnu nab@0.4.0
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install sccache

# Build application
COPY . .
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --target aarch64-unknown-linux-gnu --bin skyclaw --features discord

# ---- Export stage ----
RUN ls -la target/aarch64-unknown-linux-gnu/release/skyclaw
RUN ls -la /usr/local/cargo/bin || true
RUN ls -la /root/.cargo/bin || true
