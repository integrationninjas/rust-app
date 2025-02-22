FROM rust:1.85.0-slim-bullseye AS builder
WORKDIR /app
COPY . .
RUN cargo build --release
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-app /app/rust-app
EXPOSE 5050
CMD ["/app/rust-app"]