FROM rust:1.23.0

RUN rustup install nightly
RUN rustup override add nightly
RUN rustup component add rust-src
RUN cargo install cargo-xbuild
RUN cargo install bootimage

RUN mkdir -p /rabbit

ENV USER=sueken5
