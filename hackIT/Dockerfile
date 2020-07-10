FROM rust:1.44.1

ENV DATABASE_URL=${DATABASE_URL}

RUN USER=root cargo new --bin hack-it
WORKDIR /hack-it

ADD Cargo.lock Cargo.toml ./

RUN cargo build
RUN rm src/*.rs

ADD ./src ./src
ADD ./static ./static
ADD ./test_challenges ./test_challenges

RUN touch ./src/*
RUN touch ./.env
CMD [ "cargo", "run"]