FROM rust:1.44.1

WORKDIR /usr/src/app

# CMD cargo run

RUN cargo install systemfd cargo-watch
CMD systemfd --no-pid -s http::0.0.0.0:3000 -- cargo watch -x run