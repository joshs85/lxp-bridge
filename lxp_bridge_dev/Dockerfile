ARG BUILD_FROM
ARG BUILD_VERSION
ARG BUILD_ARCH

FROM $BUILD_FROM

# Copy the lxp-bridge binary from the published image with version
# Use BUILD_VERSION if set, otherwise default to 'dev'
COPY --from=joshs85/lxp-bridge:dev /usr/local/bin/lxp-bridge /usr/local/bin/lxp-bridge

# Install required packages (Alpine uses apk, not apt)
RUN \
  apk add --no-cache \
    yq

# Copy add-on files
COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]