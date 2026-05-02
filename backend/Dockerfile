FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml ./
COPY backend ./backend
COPY frontend ./frontend
COPY shared ./shared

RUN cargo build --release -p backend

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /usr/local/bin/backend

ENV BIND_ADDR=0.0.0.0:3000
EXPOSE 3000

CMD ["backend"]
