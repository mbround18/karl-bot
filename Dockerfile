ARG RUST_VERSION="1.72.0"
ARG DEBIAN_VERSION="12"
FROM rust:${RUST_VERSION}-bullseye AS builder

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /app

COPY . .

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-gnu --release

FROM debian:${DEBIAN_VERSION}-slim

WORKDIR /bot

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/name-bot /bot/name-bot

ENTRYPOINT [ "/bot/karl-bot" ]
