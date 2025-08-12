#! /bin/sh

case "$1" in
  "linux/amd64")
    echo x86_64-unknown-linux-musl > /rust_target.txt
    ;;
  "linux/arm64")
    echo aarch64-unknown-linux-musl > /rust_target.txt
    ;;
  "linux/arm/v7")
    echo armv7-unknown-linux-musleabihf > /rust_target.txt
    ;;
  *)
    exit 1
    ;;
esac

# Install OpenSSL development packages for cross-compilation
apk add --no-cache openssl-dev