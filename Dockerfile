FROM rust:bookworm

RUN rustup toolchain install nightly; \
    rustup default nightly; \
    rustup target add wasm32-unknown-unknown; \
    rustup component add clippy rustfmt;

RUN cargo install sccache;

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/cache

RUN cargo install cargo-leptos;