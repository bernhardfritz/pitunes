FROM rust AS chef
RUN cargo install cargo-chef
RUN apt-get update \
    && apt-get -y install --no-install-recommends \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*
RUN npm install -g yarn
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin pitunes

FROM debian:stable-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/pitunes /usr/local/bin
ENTRYPOINT ["/usr/local/bin/pitunes"]