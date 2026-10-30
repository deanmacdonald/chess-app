FROM rust:latest

WORKDIR /app

COPY . /app

RUN cargo build --release --bin chess-api

CMD ["./target/release/chess-api"]
