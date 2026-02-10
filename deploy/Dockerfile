# RustyWorm Production Dockerfile
# Multi-stage build for minimal image size

# ============================================================================
# Stage 1: Build
# ============================================================================
FROM rust:1.75-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy manifests first for layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir -p src/mimicry && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub mod mimicry;" > src/lib.rs && \
    echo "" > src/mimicry/mod.rs && \
    echo "pub mod consciousness;" >> src/lib.rs && \
    echo "pub fn prime_directive() {}" > src/consciousness.rs

# Build dependencies only
RUN cargo build --release --features full 2>/dev/null || true

# Copy actual source
COPY src/ src/

# Build the actual binary
RUN cargo build --release --features full

# ============================================================================
# Stage 2: Runtime
# ============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 -s /bin/bash rustyworm

# Create app directories
RUN mkdir -p /app /data/.rustyworm && \
    chown -R rustyworm:rustyworm /app /data

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/rustyworm /app/rustyworm

# Copy any configuration files
COPY deploy/config.toml /app/config.toml 2>/dev/null || true

# Set ownership
RUN chown -R rustyworm:rustyworm /app

USER rustyworm

# Environment variables
ENV RUSTYWORM_DATA_DIR=/data/.rustyworm
ENV RUSTYWORM_LOG_LEVEL=info
ENV RUST_BACKTRACE=1

# Expose ports (if running in server mode)
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD /app/rustyworm --version || exit 1

# Default command - interactive REPL
ENTRYPOINT ["/app/rustyworm"]
CMD []
