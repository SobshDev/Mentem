# syntax=docker/dockerfile:1
FROM rust:1-slim AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false app

WORKDIR /app
COPY --from=builder /app/target/release/api /app/api
USER app
EXPOSE 3000
CMD ["/app/api"]
