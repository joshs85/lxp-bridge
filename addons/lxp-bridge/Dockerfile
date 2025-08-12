ARG BUILD_FROM
FROM $BUILD_FROM

# Install required packages
RUN apk add --no-cache yq

# Copy the binary and scripts
COPY run.sh /
RUN chmod a+x /run.sh

# Set the entrypoint
CMD [ "/run.sh" ]