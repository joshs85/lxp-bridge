# https://hub.docker.com/repository/docker/joshs85/lxp-bridge
#
# Building/publishing:
# docker build -t joshs85/lxp-bridge .
# docker push joshs85/lxp-bridge:latest
#

FROM --platform=$BUILDPLATFORM rust:1.89-alpine3.22 AS builder
WORKDIR /usr/src/lxp-bridge
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src
RUN cargo install --path .


FROM alpine:3.22
RUN apk add --no-cache libssl3
COPY --from=builder /usr/local/cargo/bin/lxp-bridge /usr/local/bin/lxp-bridge
ENTRYPOINT ["lxp-bridge", "-c", "/etc/config.yaml"]
