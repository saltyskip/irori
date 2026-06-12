FROM rust:1.81 as builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY server/ server/
COPY cli/ cli/

# Build server
RUN cargo build --release -p hearth-server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/hearth /app/hearth

EXPOSE 3000

CMD ["/app/hearth"]
