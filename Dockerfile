FROM alpine:3.20

# Curl version should not matter
# hadolint ignore=DL3018
RUN apk update && \
    apk upgrade && \
    apk add --no-cache curl && \
    apk cache clean

COPY ./www /www
COPY ./target/x86_64-unknown-linux-musl/release/docker-swarm-visualizer-rs /docker-swarm-visualizer-rs

HEALTHCHECK \
    --interval=60s \
    --timeout=1s \
    --start-period=30s \
    --retries=3 \
    CMD curl -f http://localhost:3500/healthcheck || exit 1

CMD ["/docker-swarm-visualizer-rs"]
