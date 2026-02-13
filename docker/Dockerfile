# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binary
RUN cargo build --release -p ulw-cli

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create ulw user
RUN useradd -m -u 1000 ulw && \
    mkdir -p /home/ulw/.ulw && \
    chown -R ulw:ulw /home/ulw

# Copy binary from builder
COPY --from=builder /app/target/release/ulw /usr/local/bin/

USER ulw
WORKDIR /home/ulw

# Default command
ENTRYPOINT ["ulw"]
CMD ["--help"]
