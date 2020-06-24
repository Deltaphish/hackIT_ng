FROM rust:1.44.1

ENV DATABASE_URL=${DATABASE_URL}

RUN USER=root cargo new --bin hack-it
WORKDIR /hack-it

ADD Cargo.lock Cargo.toml ./

RUN cargo build --release
RUN rm src/*.rs

ADD ./src ./src
ADD ./static ./static

RUN touch ./src/*

CMD [ "cargo", "run", "--release" ]