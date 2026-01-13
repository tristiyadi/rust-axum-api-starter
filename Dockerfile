# Stage 1: Builder
FROM rust:1.92.0-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    musl-tools \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Add musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features mysql

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release with musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime
FROM gcr.io/distroless/static:nonroot

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend-api-jwt /app/backend-api-jwt

# Copy migrations
COPY migrations ./migrations

# Expose port
EXPOSE 3000

# Run the binary
USER nonroot
CMD ["./backend-api-jwt"]
