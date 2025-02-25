# stage 1
FROM rust:1.84.1-bullseye as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo fetch
RUN cargo build --release
#stage 2
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-app /app/rust-app
EXPOSE 5050
CMD [ "/app/rust-app" ]