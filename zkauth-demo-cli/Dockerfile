FROM rust:1.76 as builder
WORKDIR /code
COPY . .
RUN cargo install --path zkauth-demo-cli

FROM debian:stable-slim
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt update && \
    apt install -y -q --no-install-recommends ca-certificates apt-transport-https && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/zkauth-demo-cli /usr/local/bin/zkauth-demo-cli
RUN useradd -s /dev/null none
ENV USER=none
USER none
ENTRYPOINT [ "zkauth-demo-cli" ]
CMD [ "--help" ]
