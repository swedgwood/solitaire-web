FROM rust:1.76-buster AS builder

RUN rustup target add wasm32-unknown-unknown && \
    # cargo install wasm-pack && \
    cargo install trunk wasm-bindgen-cli && \
    apt update && apt install -y binaryen

WORKDIR /app/
COPY . /app/
RUN trunk build --release

FROM caddy

COPY --from=builder /app/dist /usr/share/caddy