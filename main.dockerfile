FROM rust:latest as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM ghcr.io/malken21/docker-minecraft-paper:1.20.4-3
COPY --from=build-env /app/target/release/server-maintainer /