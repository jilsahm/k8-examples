FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/redis-app /usr/local/bin/redis-app
ENTRYPOINT [ "redis-app" ]