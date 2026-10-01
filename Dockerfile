# --- Stage 1: Build ----------------------------------------------------------
FROM rustlang/rust:nightly AS builder
WORKDIR /app

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/Cargo.toml

# Pre-build dependency graph for caching
RUN mkdir -p api/src && echo "fn main() {}" > api/src/main.rs
RUN cargo build --release -p api || true

# Copy real API source
COPY api ./api

# Build the real API binary (glibc target)
RUN cargo build --release -p api

# --- Stage 2: Runtime --------------------------------------------------------
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the API binary from the builder stage
COPY --from=builder /app/target/release/api /app/api

ENV PORT=8000
CMD ["./api"]

