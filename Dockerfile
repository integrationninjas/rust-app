# Use a minimal Rust base image (Alpine) for building
FROM rust:1.84.1-alpine AS builder

# Install dependencies (musl-tools for Alpine)
RUN apk add --no-cache musl-dev

# Set working directory
WORKDIR /app

# Copy only Cargo files first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies without compiling the full project
RUN cargo fetch

# Copy the actual source code
COPY . .

# Build the release binary
RUN cargo build --release

# Use Alpine as a lightweight runtime environment
FROM alpine:latest

# Set working directory
WORKDIR /app

# Install required runtime dependencies (if needed)
RUN apk add --no-cache ca-certificates

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rust-app /app/rust-app

# Expose the port
EXPOSE 5050

# Run the application
CMD ["/app/rust-app"]
