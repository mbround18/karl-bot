# Versions of dependancies
ARG RUST_VERSION="1.72.0"
ARG DEBIAN_VERSION="12"

# Set the root container
FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION} AS chef

WORKDIR /app

# Copy the manifest and lock file, compile to recipe
FROM chef as planner

RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY ./Cargo.lock ./Cargo.toml ./

RUN cargo chef prepare --recipe-path recipe.json

# Build the project
FROM chef AS builder

# Copy recipe from planner
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN rustup target add x86_64-unknown-linux-gnu \
    && cargo build --release  \
    && rm -rf src/ target/release/deps/

# Use a slim container for the final image
FROM debian:${DEBIAN_VERSION}-slim

WORKDIR /bot

COPY --from=builder /app/target/release/name-bot /bot/name-bot
COPY ./scripts/entrypoint.sh /bot/entrypoint.sh
RUN chmod +x /bot/entrypoint.sh

ENTRYPOINT ["/bot/entrypoint.sh"]
