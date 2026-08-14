# Multi-stage Dockerfile for Project Sunrise Destiny 2 Emulation Server
# Stage 1: Build static binary with Rust Alpine
FROM rust:alpine AS builder

WORKDIR /build

# Install required build utilities
RUN apk add --no-cache musl-dev

# Copy Linux crate files
COPY Linux/Cargo.toml Linux/Cargo.lock ./
COPY Linux/assets ./assets
COPY Linux/src ./src

# Build release binary
RUN cargo build --release --locked

# Stage 2: Minimal, secure runtime container
FROM alpine:3.20

LABEL maintainer="UberMetroid <https://github.com/UberMetroid/Sunrise>"
LABEL description="Project Sunrise - Standalone Destiny 2 Emulation Server Container"

# Create non-root sunrise user & data directory
RUN addgroup -S sunrise && adduser -S sunrise -G sunrise \
    && mkdir -p /data \
    && chown -R sunrise:sunrise /data

# Copy compiled binary from builder
COPY --from=builder /build/target/release/sunrise-linux /usr/local/bin/sunrise-linux

# Set permissions
RUN chmod +x /usr/local/bin/sunrise-linux

# Default environment configuration
ENV SUNRISE_BIND_ADDRESS=0.0.0.0 \
    SUNRISE_PORT=7777 \
    RUST_LOG=info

# Expose BAP emulation server port
EXPOSE 7777/tcp

# Persistent storage volume for configurations and player accounts
VOLUME ["/data"]

USER sunrise
WORKDIR /data

# Healthcheck to monitor port availability
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD nc -z 127.0.0.1 7777 || exit 1

ENTRYPOINT ["sunrise-linux", "server", "0.0.0.0", "7777"]
