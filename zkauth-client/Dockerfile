FROM rust:1.76 as builder
WORKDIR /code
COPY . .
RUN cargo install --path zkauth-client

FROM debian:stable-slim
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt update && \
    apt install -y -q --no-install-recommends ca-certificates apt-transport-https && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/zkauth-client /usr/local/bin/zkauth-client
RUN useradd -s /dev/null none
ENV USER=none
USER none
ENTRYPOINT [ "zkauth-client" ]
CMD [ "--help" ]
