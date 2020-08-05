FROM rust:1.44.1

WORKDIR /usr/src/app

RUN rustup default nightly
RUN cargo install cargo-watch

CMD cargo watch -x run