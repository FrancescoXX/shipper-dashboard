FROM rust:1-bookworm AS builder
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown \
    && cargo install trunk --locked

COPY Cargo.toml ./
COPY backend ./backend
COPY frontend ./frontend
COPY shared ./shared

WORKDIR /app/frontend
RUN trunk build --release

FROM nginx:1.27-alpine
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

EXPOSE 80
