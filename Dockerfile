FROM ekidd/rust-musl-builder AS chef
USER root
RUN cargo install cargo-chef \
    && rm -rf $CARGO_HOME/registry
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash - \
    && curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor | sudo tee /usr/share/keyrings/yarnkey.gpg >/dev/null \
    && echo "deb [signed-by=/usr/share/keyrings/yarnkey.gpg] https://dl.yarnpkg.com/debian stable main" | sudo tee /etc/apt/sources.list.d/yarn.list \
    && apt-get update \
    && apt-get -y install yarn \
    && rm -rf /var/lib/apt/lists/*
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