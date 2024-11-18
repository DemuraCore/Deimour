# Build stage
FROM rust:1.81 as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create build directory
WORKDIR /usr/src/deimour

# Build release version
RUN cargo build --release

# Runtime stage
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/deimour/target/release/deimour /usr/local/bin/deimour

# Set environment variables
ENV RUST_LOG=info

# Create data directory for config
WORKDIR /data


# Run as non-root user
RUN useradd -m deimour
USER deimour

# Command to run
CMD ["deimour"]