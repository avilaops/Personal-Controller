# Multi-stage build for optimized production image
FROM rust:1.83-slim as builder

WORKDIR /app

# Install dependencies for compilation
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY pc-core ./pc-core
COPY pc-models ./pc-models
COPY pc-db ./pc-db
COPY pc-llm ./pc-llm
COPY pc-api ./pc-api

# Build in release mode
RUN cargo build --release --package pc-api

# Runtime stage - minimal image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/pc-server /app/pc-server

# Create non-root user
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /app

USER appuser

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/usr/bin/curl", "-f", "http://localhost:3000/health", "||", "exit 1"]

CMD ["/app/pc-server"]
