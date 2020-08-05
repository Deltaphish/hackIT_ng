FROM rust:1.44.1

WORKDIR /usr/src/app

RUN rustup default nightly
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

CMD bash -c "bash ./wait-for-it.sh postgresql:5432 -q -- diesel migration run && cargo watch -x run"