FROM ekidd/rust-musl-builder AS chef
USER root
RUN cargo install cargo-chef \
    && rm -rf $CARGO_HOME/registry
RUN apt-get update \
    && apt-get -y install --no-install-recommends \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*
RUN npm set unsafe-perm true
RUN npm install -g yarn
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin pitunes

FROM alpine as runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pitunes /usr/local/bin
ENTRYPOINT ["/usr/local/bin/pitunes"]