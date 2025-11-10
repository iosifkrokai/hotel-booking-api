# Build stage
FROM rust:1.91-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src

RUN apt-get update && apt-get install -y pkg-config libssl-dev curl
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

ENV APP_PORT=${APP_PORT}

RUN apt-get update && apt-get install -y ca-certificates
WORKDIR /app

COPY --from=builder /app/target/release/hotel-booking-api /app/hotel-booking-api
COPY --from=builder /app/migrations ./migrations

EXPOSE ${APP_PORT}

CMD ["./hotel-booking-api"]
