FROM ekidd/rust-musl-builder AS builder
USER root
RUN apt-get update \
    && apt-get -y install --no-install-recommends \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*
RUN npm set unsafe-perm true
RUN npm install -g yarn
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin pitunes

FROM alpine as runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pitunes /usr/local/bin
ENTRYPOINT ["/usr/local/bin/pitunes"]