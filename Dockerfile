FROM rust:1.30-stretch as builder

ADD . ./

RUN apt update && \
    apt install -y libssl-dev && \
    cargo build --verbose --release && \
    cargo install

FROM debian:stretch
COPY --from=builder /usr/local/cargo/bin/dash_manifest_worker /usr/bin

RUN apt update && apt install -y libssl1.1 ca-certificates

ENV AMQP_QUEUE job_dash_manifest
ENV AMQP_COMPLETED_QUEUE job_dash_manifest_completed
ENV AMQP_ERROR_QUEUE job_dash_manifest_error
CMD dash_manifest_worker
