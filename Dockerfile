FROM rust:1.84.1-bullseye AS builder

WORKDIR /app

# Copy only Cargo.toml and Cargo.lock first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source directory
COPY src ./src

# Download dependencies
RUN cargo fetch

# Build the application
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/rust-app /app/rust-app

EXPOSE 5050

CMD ["/app/rust-app"]
