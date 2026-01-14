FROM rust:1.90-slim-bookworm AS builder

ARG DATABASE_BACKEND=sqlite

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY api-types/ ./api-types/
COPY api/ ./api/

WORKDIR /app/api
RUN cargo build --release --features ${DATABASE_BACKEND}

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/api/target/release/api-server /usr/local/bin/api-server

WORKDIR /app

RUN mkdir -p /app/data

ENV BIND_ADDRESS=0.0.0.0:8080
ENV DATABASE_URL=sqlite:/app/data/app.db?mode=rwc

EXPOSE 8080

CMD ["api-server"]
