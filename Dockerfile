FROM rust:1.72.0-bullseye AS builder

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /app

COPY . .

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-gnu --release

FROM debian:bookworm

WORKDIR /bot

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/karl-bot /bot/karl-bot

ENTRYPOINT [ "/bot/karl-bot" ]
