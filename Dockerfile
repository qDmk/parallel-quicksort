FROM rust:1.90-alpine

RUN apk add build-base

RUN mkdir /quicksort
COPY . /quicksort
WORKDIR /quicksort
RUN cargo build --release --lib --benches --tests

CMD cargo test --release && cargo bench
