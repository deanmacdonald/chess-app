FROM rust:1.84 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/chess-api /app/chess-api
ENV PORT=8000
CMD ["./chess-api"]
