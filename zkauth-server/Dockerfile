FROM rust:1.76 as builder
WORKDIR /code
COPY . .
RUN cargo install --path zkauth-server

FROM debian:stable-slim
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt update && \
    apt install -y -q --no-install-recommends ca-certificates apt-transport-https && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/zkauth-server /usr/local/bin/zkauth-server
RUN useradd -s /dev/null none
ENV USER=none
USER none
ENV PORT=5000
EXPOSE $PORT
CMD ["zkauth-server", "--host", "0.0.0.0"]
