FROM rust:1.41-stretch as builder

ADD . /src
WORKDIR /src

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    cargo build --verbose --release && \
    cargo install --path .

FROM debian:stretch
COPY --from=builder /usr/local/cargo/bin/dash_manifest_worker /usr/bin

RUN apt-get update && apt install -y libssl1.1 ca-certificates

ENV AMQP_QUEUE job_dash_manifest
CMD dash_manifest_worker
