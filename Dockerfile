# ---- Chef stage (Base environment) ----
# Pinned to rust:1.93 digest to ensure cache stability
FROM rust:1.93 AS chef
RUN cargo install --locked cargo-chef --version 0.1.68
WORKDIR /app

# Install cross-compilation tools for aarch64 (Cached layer)
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu libc6-dev-arm64-cross

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
RUN cargo chef cook --release --target aarch64-unknown-linux-gnu --recipe-path recipe.json

# Pre-install nab binary for deployment (Cached unless version changes)
RUN cargo install nab --version 0.4.0 --target aarch64-unknown-linux-gnu

# Build application
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-gnu --bin skyclaw

# ---- Export stage ----
RUN ls -la target/aarch64-unknown-linux-gnu/release/skyclaw
RUN ls -la /usr/local/cargo/bin/nab
