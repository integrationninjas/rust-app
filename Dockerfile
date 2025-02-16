# Use Debian-based Rust image for building to match runtime environment
FROM rust:1.84.1-bullseye AS builder

# Set working directory
WORKDIR /app

# Copy actual source code
COPY . .

# Pre-build dependencies
RUN cargo build --release

# Use the same Debian-based image to prevent GLIBC issues
FROM debian:bullseye

# Set working directory
WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/rust-app /app/rust-app

# Expose the port
EXPOSE 5050

# Run the application
CMD ["/app/rust-app"]
