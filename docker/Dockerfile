FROM ghcr.io/bluebastion/dev-bb-rs-builder:v1.0.0 as webhook
ARG RUST_BIN="rs-splunk-webhook"
ENV RUST_BIN=$RUST_BIN
ENV RUST_LOG="debug"
ENV SERVICE_NAME="rs-splunk-webhook"
ENV SERVICE_VERSION="0.0"
RUN install -d -o 10001 -g 10001 /app
RUN install -d -o 10001 -g 10001 /app/logs
COPY --chown=10001:10001 --chmod=0744 ./release/$RUST_BIN /usr/local/bin/$RUST_BIN
WORKDIR /app
USER 10001
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD [ "/bin/bash", "-c", "curl http://127.0.0.1:4200" ]
ENTRYPOINT [ "/bin/bash", "-c", "$RUST_BIN $@"]
