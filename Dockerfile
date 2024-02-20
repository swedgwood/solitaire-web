ARG PUBLIC_URL="/solitaire"

FROM rust:1.76-buster AS builder


RUN rustup target add wasm32-unknown-unknown && \
    # cargo install wasm-pack && \
    cargo install trunk wasm-bindgen-cli && \
    apt update && apt install -y binaryen

WORKDIR /app/
COPY . /app/

ARG PUBLIC_URL

RUN trunk build --release --public-url "${PUBLIC_URL}"

FROM caddy

RUN rm -rf /usr/share/caddy/*
ARG PUBLIC_URL
COPY --from=builder /app/dist "/usr/share/caddy/${PUBLIC_URL}"
