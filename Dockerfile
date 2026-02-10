# RustyWorm Docker Image
# Build:  docker build -t rustyworm .
# Run:    docker run --rm -it -v $(pwd)/data:/data rustyworm
# With API keys:
#   docker run --rm -it -e OPENAI_API_KEY=sk-... rustyworm

# Stage 1: Build
FROM rust:1.93-bookworm AS builder

WORKDIR /build

# Copy manifests first for dependency layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy sources to pre-compile dependencies
RUN mkdir -p src/mimicry && \
    echo "pub mod consciousness; pub mod mimicry; pub mod ising_empathy;" > src/lib.rs && \
    echo "fn main() {}" > src/main.rs && \
    touch src/consciousness.rs src/ising_empathy.rs && \
    for m in mod analyzer cache capability engine evolution persistence profile templates; do \
        touch "src/mimicry/${m}.rs"; \
    done && \
    cargo build --release --features api 2>/dev/null || true && \
    rm -rf src/

# Copy real source code
COPY src/ src/
COPY consciousness_test.rs unified_test.rs self_reference_test.rs \
     infinite_recursion_test.rs awakening_test.rs comprehensive_test.rs \
     prime_directive.rs ./

# Build the release binary
RUN cargo build --release --features api --bin rustyworm

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/rustyworm /usr/local/bin/rustyworm

# Persistence directory
RUN mkdir -p /data/.rustyworm
WORKDIR /data

ENTRYPOINT ["rustyworm"]
