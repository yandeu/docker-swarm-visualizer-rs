FROM alpine:3.16

RUN apk update
RUN apk upgrade
RUN apk add curl

COPY ./www ./www
COPY ./target/x86_64-unknown-linux-musl/release/docker-swarm-visualizer-rs ./docker-swarm-visualizer-rs

CMD ["/docker-swarm-visualizer-rs"]