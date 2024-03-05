# Zero Knowledge Authentication

This repository provides a Rust library implementing the Chaum-Pedersen protocol, enabling the creation of zero-knowledge proofs for cryptographic verification while ensuring privacy. The project implements two flavors of the Chaum-Pedersen cryptographic proofs; one using discrete logarithms and the other using elliptive curves. These mechanisms allow a prover to demonstrate knowledge of a secret corresponding to a public value without revealing the secret itself. The project also includes a gRPC server and client implementation for integration into applications that wish to utilize the protocol to register and authenticate users.

[![codecov](https://codecov.io/gh/snormore/zkauth/graph/badge.svg?token=JN2KIKA175)](https://codecov.io/gh/snormore/zkauth)

[Docs](https://snormore.github.io/zkauth)

## Features

- Chaum-Pedersen protocol-based gRPC authentication service.
- Discrete logarithm flavor using BigInt via [num-bigint](https://github.com/rust-num/num-bigint).
- Elliptic curve flavor using Ristretto points with [curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek).
- gRPC client/server for protocol operations.
- Server and CLI client.
- External user storage and challenge data cache.
- Docker setup for local testing.
- Terraform for AWS deployment.

## Project Layout

- [`zkauth`](./zkauth): Core library implementing the [`discrete_logarithm`](./zkauth/src/discrete_logarithm) and [`elliptic_curve`](./zkauth/src/elliptic_curve) flavors of the protocol, with operations defined in `prover.rs` and `verifier.rs` for each.
- [`zkauth-protobuf`](./zkauth-protobuf): Generated protobuf types and stubs for the gRPC service.
- [`zkauth-server`](./zkauth-server): Implementation of the gRPC service as defined in [`zkauth-protobuf/v1.proto`](./zkauth-protobuf/v1.proto), and a binary definition used for execution of the server. This component acts as the verifier in the Chaum-Pedersen protocol.
- [`zkauth-client`](./zkauth-client): Implementation of the gRPC service client as defined in [`zkauth-protobuf/v1.proto`](./zkauth-protobuf/v1.proto). This component acts as the prover in the Chaum-Pedersen protocol.
- [`zkauth-demo-cli`](./zkauth-demo-cli): A simple command-line interface for acting as the client, and hence the prover, to interact with the gRPC service.
- [`tests`](./zkauth): A suite of functional tests that encode the expectations of the client/prover and server/verifier in an end-to-end way.

## Chaum-Pedersen Proofs

The library supports two flavors:

### Discrete Logarithm

The classic Chaum-Pedersen protocol is a cryptographic technique mainly used for proving that two discrete logarithms are equal and that they correspond to the same base without revealing the actual values. This protocol is commonly utilized in privacy-preserving cryptographic systems such as electronic voting schemes and zero-knowledge proof constructions.

Here are the steps of the Chaum-Pedersen protocol:

1. **Setup**: The prover and verifier agree on a prime $p$ and a generator $g$ of a cyclic group $G$ of order $q$, where $q$ is a large prime factor of $p-1$. The prover knows a secret $x$, which is the discrete logarithm of both $y_1 = g^x \mod p$ and $y_2 = h^x \mod p$ to the bases $g$ and $h$, respectively. Note that $h$ is another element of $G$, and the equality of logarithms $\log_g(y_1) = \log_h(y_2) = x$ is what the prover intends to prove without revealing $x$.
2. **Commitment**: The prover selects a random value $k$ from the group $G$ and computes two commitments $r_1 = g^k \mod p$ and $r_2 = h^k \mod p$. The prover then sends the commitments $r_1$ and $r_2$ to the verifier.
3. **Challenge**: The verifier sends a random challenge $c$ to the prover. This challenge is typically a random number selected from a range that ensures security, such as the order of the group $q$.
4. **Response**: Upon receiving the challenge $c$, the prover computes the response $s = k - c \cdot x \mod q$ and sends $s$ to the verifier.
5. **Verification**: The verifier checks the validity of the prover's response by ensuring that both $r_1 = g^s \cdot y_1^c \mod p$ and $r_2 = h^s \cdot y_2^c \mod p$ hold true. If both equations are satisfied, the verifier accepts the proof; otherwise, the proof is rejected.

The protocol ensures that the prover knows the discrete logarithm $x$ without revealing it. The security of the protocol relies on the difficulty of computing discrete logarithms in the group $G$.

### Elliptic Curve

Adapting the Chaum-Pedersen protocol to elliptic curves involves leveraging the elliptic curve discrete logarithm problem (ECDLP) instead of the classical discrete logarithm problem in a cyclic group. The fundamental principles remain similar, but the operations are adapted to the properties and operations of elliptic curves.

Here's how the steps adapt:

1. **Setup**: Instead of agreeing on a prime $p$ and a generator $g$ of a cyclic group, the prover and verifier agree on an elliptic curve $E$ defined over a finite field and a base point $G$ on $E$ of prime order $q$. The prover knows a secret scalar $x$, which corresponds to the discrete logarithm (with respect to base point $G$) of two points $Y_1 = xG$ and $Y_2 = xH$ on the elliptic curve, where $H$ is another point on the curve. The prover intends to demonstrate that $\log_G(Y_1) = \log_H(Y_2) = x$ without revealing $x$.
2. **Commitment**: The prover picks a random scalar $k$ from the set $1, ..., q-1$ and computes two commitment points $R_1 = kG$ and $R_2 = kH$ on the elliptic curve. These commitments $R_1$ and $R_2$ are then sent to the verifier.
3. **Challenge**: The verifier generates a random challenge scalar $c$ and sends it to the prover. This challenge is again a random scalar from the set $1, ..., q-1$.
4. **Response**: Upon receiving $c$, the prover calculates the response scalar $s = k + cx \mod q$ and sends $s$ back to the verifier.
5. **Verification**: The verifier receives $s$ and validates the prover’s claims by checking if $sG = R_1 + cY_1$ and $sH = R_2 + cY_2$ on the elliptic curve, or equivalently if $R_1 = sG - cY_1$ and $R_2 = sH - cY_2$. If both equations hold, the prover's claim is accepted; otherwise, it is rejected.

Adapting the protocol to elliptic curves maintains the privacy and security characteristics of the original Chaum-Pedersen protocol while leveraging the added security benefits and efficiency of elliptic curve cryptography, which typically allows for shorter key sizes compared to traditional discrete logarithm-based systems for a comparable level of security. The main changes involve moving from multiplicative group operations to additive elliptic curve group operations and from working with integers modulo a prime to working with points on an elliptic curve.

## Getting Started

### Local command-line

This project is written in Rust, which should be available on your system.

Clone the repository:

```sh
git clone https://github.com/snormore/zkauth.git
cd zkauth
```

Run the tests:

```sh
cargo test
```

Run lint checks:

```sh
cargo clippy --all-features --no-deps
```

or

```sh
dev/lint
```

Build the libraries and binaries:

```sh
cargo build
```

### Usage

#### Server

```
$ cd zkauth-server
$ cargo run -- --help

Usage: zkauth-server [OPTIONS]

Options:
  -v, --verbose...
          Increase logging verbosity
  -q, --quiet...
          Decrease logging verbosity
      --host <HOST>
          Specifies the IP address or name of the host to which the server is bound [default: 127.0.0.1]
  -p, --port <PORT>
          Specifies the TCP/IP port number on which the server listens for incoming client requests [env: PORT=] [default: 0]
      --config-path <CONFIG_PATH>
          Specifies the configuration file path. If not specified, a non-persistent configuration will be generated and used [env: CONFIG_PATH=]
      --config-generate
          Specifies whether to generate a new configuration file at the specified path. If true, this will exit after generating the configuration file, and not run the server. If the file already exists, it will not be overwritten unless the --config-overwrite is specified
      --config-overwrite
          Specifies whether to overwrite an existing configuration file when generating a new one
      --config-flavor <CONFIG_FLAVOR>
          Specifies the configuration flavor to use [default: discrete-logarithm] [possible values: discrete-logarithm, elliptic-curve]
      --config-prime-bits <CONFIG_PRIME_BITS>
          Specifies the number of bits to use for generating prime numbers for the public parameters [default: 256]
      --config-prime <CONFIG_PRIME>
          Specifies a prime number to use for generating the configuration
  -h, --help
          Print help
  -V, --version
          Print version
```

The server can be run with the following command:

```sh
zkauth-server --port 50001
```

The server can generate a configuration file using the following command:

```sh
zkauth-server --config-generate --config-path=config.json
```

You can specify the configuration flavor using the `--config-flavor` option, and the number of bits for the prime number using the `--config-prime-bits` option, or specify a prime number directly using the `--config-prime` option.

```sh
zkauth-server --config-generate --config-path=config.json --config-flavor=elliptic-curve
```

```sh
zkauth-server --config-generate --config-path=config.json --config-prime-bits=256
```

```sh
zkauth-server --config-generate --config-path=config.json --config-prime=42765216643065397982265462252423826320512529931694366715111734768493812630447
```

#### Demo Client CLI

```
$ cd zkauth-demo-cli
$ cargo run -- --help

Usage: zkauth-demo-cli [OPTIONS] --address <ADDRESS> --user <USER> --password <PASSWORD>

Options:
  -v, --verbose...           Increase logging verbosity
  -q, --quiet...             Decrease logging verbosity
  -a, --address <ADDRESS>    Specifies the address of the gRPC server to connect to. Example: http://127.0.0.1:50001 [env: ZKAUTH_ADDRESS=]
  -u, --user <USER>          Specifies the username to authenticate with [env: ZKAUTH_USER=]
  -p, --password <PASSWORD>  Specifies the password to authenticate with [env: ZKAUTH_PASSWORD=]
      --register             Specifies whether to execute the registration step
      --login                Specifies whether to execute the login step
  -h, --help                 Print help
  -V, --version              Print version
```

Execute the register and login workflows against the server:

```sh
cargo run -- --address http://localhost:50001 --user user --password password --register --login
```

### Local docker-compose

Build and spin up the docker containers for the server and demo CLI:

```sh
docker-compose up -d
```

List the containers:

```sh
docker-compose ps -a
```

View the logs:

```sh
docker-compose logs -f
```

Run the demo client CLI with arguments:

```sh
docker-compose run --rm cli --user user --password password --register --login
```

If you need to rebuild the containers after changing the code, you can run

```sh
docker-compose build
docker-compose up -d
```

or

```sh
docker-compose up -d --build
```

## Future Work

The project can be considered a proof-of-concept, and does not yet have certain features that would be required for production use:

- Users are stored in memory in a hash map in the gRPC service instance, and so will not persist across restarts. This should ideally be stored in a more stable, permanant location, such as a SQL DB or external key value store. This would also allow for high-availability of the server instances, with support for multiple stateless instances.
- Challenges and sessions are stored in memory in a TTL cache, and so will not persist across restarts. This should ideally be stored in a more stable location, such as Redis or a caching key value store, so that multiple instances of the server can interace for high-availability and better scalability.

## License

This project is licensed under the [MIT license](LICENSE).

## Acknowledgements

- [Wallet Databases with Observers](https://link.springer.com/content/pdf/10.1007/3-540-48071-4_7.pdf) by David Chaum and Torben Pryds Pedersen
- [Cryptography: An Introduction](https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf) by Nigel Smart
- [Chaum-Pedersen Protocol](https://en.wikipedia.org/wiki/Publicly_Verifiable_Secret_Sharing#Chaum-Pedersen_Protocol) on Wikipedia
- [Chaum-Pedersen Protocol](https://crypto.stackexchange.com/questions/99262/chaum-pedersen-protocol) question on StackExchange
- [Chaum-Pedersen protocol adapted to elliptic curves](https://crypto.stackexchange.com/questions/105889/chaum-pedersen-protocol-adapted-to-elliptic-curves?noredirect=1#comment226693_105889) question on StackExchange
- Existing, similar projects:
  - https://github.com/twilker/cp-zkp
  - https://github.com/kobby-pentangeli/chaum-pedersen-zkp
  - https://github.com/adrianperezkeilty/Chaum-Pedersen-Protocol
  - https://github.com/gagiuntoli/zkp_chaum_pedersen
  - https://github.com/neongazer/zkp-auth-py
  - https://github.com/SoftwareSecurityLab/Chaum-Pedersen
  - https://github.com/georgemakrakis/Chaum-Pedersen_NI-ZKP

## Appendix

### Login process using discrete logarithm

![diagram](https://i.stack.imgur.com/pNiFt.png)
