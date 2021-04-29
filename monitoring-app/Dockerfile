FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/monitoring-app /usr/local/bin/monitoring-app
ENTRYPOINT [ "monitoring-app" ]