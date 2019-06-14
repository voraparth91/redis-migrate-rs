FROM rustlang/rust:nightly as builder
WORKDIR /app/
COPY ./ ./

RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app

COPY --from=builder /app/target/release/redis-copy-rs ./

CMD ["/app/redis-copy-rs"]