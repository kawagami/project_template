FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

COPY src/ src/
COPY Cargo.toml .
COPY .env .

RUN cargo build --release

# 漸少 image size
RUN strip -s /app/target/release/backend

# 好像 sqlx 有用到不能用 scratch 的依賴
FROM ubuntu:22.04

COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/.env /app/.env

CMD ["/app/backend"]