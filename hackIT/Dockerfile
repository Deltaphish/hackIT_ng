FROM rust:1.44.1

ENV DATABASE_URL=${DATABASE_URL}

RUN USER=root cargo new --bin hackIT
WORKDIR /hackIT

ADD Cargo.lock Cargo.toml ./

RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/hackIT*

ADD ./src ./src
ADD ./static ./static

CMD [ "cargo", "run", "--release" ]