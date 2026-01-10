# Build stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/riwaq

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev git && \
    rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build release binary
WORKDIR /usr/src/riwaq/core
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install system dependencies needed for git2 and general operation
RUN apt-get update && \
    apt-get install -y ca-certificates git libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /usr/src/riwaq/core/target/release/riwaq /usr/local/bin/riwaq

# Create directory for mounting host projects
RUN mkdir -p /data

# Set environment variables
ENV RIWAQ_HOST=0.0.0.0
ENV RIWAQ_PORT=9527

# Expose server port
EXPOSE 9527

# Check health
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:9527/health || exit 1

# Default command starts the server
ENTRYPOINT ["riwaq"]
CMD ["serve", "--host", "0.0.0.0", "--port", "9527"]
