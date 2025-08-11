ARG BUILD_FROM

FROM joshs85/lxp-bridge:latest AS lxp-bridge

FROM $BUILD_FROM
COPY --from=lxp-bridge / /

RUN apk add --no-cache yq

COPY run.sh /
RUN chmod a+x /run.sh

CMD [ "/run.sh" ]