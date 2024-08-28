# https://hub.docker.com/repository/docker/joshs85/lxp-bridge
#
# Building/publishing:
# docker build -t joshs85/lxp-bridge .
# docker push joshs85/lxp-bridge:latest
#

FROM rust:latest as builder
WORKDIR /usr/src/lxp-bridge
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src
COPY db db
RUN cargo install --path .


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/lxp-bridge /usr/local/bin/lxp-bridge
ENTRYPOINT ["lxp-bridge", "-c", "/etc/config.yaml"]
