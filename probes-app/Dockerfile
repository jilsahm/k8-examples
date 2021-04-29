FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/probes-app /usr/local/bin/probes-app
ENTRYPOINT [ "probes-app" ]