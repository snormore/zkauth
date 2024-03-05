# Zero Knowledge Authentication

This repository provides a Rust library implementing the [Chaum-Pedersen protocol](https://en.wikipedia.org/wiki/Publicly_Verifiable_Secret_Sharing#Chaum-Pedersen_Protocol), enabling the creation of zero-knowledge proofs for cryptographic verification while ensuring privacy. It includes a gRPC server and client that build on the library, allowing for integration into applications that utilize the protocol to register and authenticate users. While the Chaum-Pedersen protocol was initially designed for discrete logarithms, this implementation also supports elliptic curves.

[![codecov](https://codecov.io/gh/snormore/zkauth/graph/badge.svg?token=JN2KIKA175)](https://codecov.io/gh/snormore/zkauth)

[Crate Docs](https://snormore.github.io/zkauth)

## Overview

The [Chaum-Pedersen protocol](https://en.wikipedia.org/wiki/Publicly_Verifiable_Secret_Sharing#Chaum-Pedersen_Protocol) is used to securely prove that the discrete logarithms of two numbers with respect to different bases are equal, without revealing the actual logarithms. Below is a step-by-step breakdown of the protocol:

1. Setup:

   - Let there be a finite group `G` with a common known base `g` and `h`.
   - Assume values `y = g^x` and `z = h^x` for some secret `x`.
   - The goal is to prove that the same `x` is used in both `y` and `z` without revealing `x`.

2. Commitment:

   - The prover selects a random value `k`.
   - Computes `a = g^k` and `b = h^k`.
   - Sends `a` and `b` to the verifier as the commitment.

3. Challenge:

   - The verifier sends a random number `c` to the prover as a challenge.

4. Response:

   - The prover computes the response `r = k + cx` (operations are done in the field corresponding to `G`).
   - Sends `r` back to the verifier.

5. Verification:
   - The verifier checks whether `g^r = a * y^c` and `h^r = b * z^c`.
   - If both equations hold, the verifier is convinced that the prover knows `x`.

This protocol ensures the prover cannot cheat without knowing the discrete logarithm `x`, and the verifier learns nothing about `x` except whether the prover knows it. The zero-knowledge aspect ensures that no additional information about `x` is revealed.

## Features

- Chaum-Pedersen non-interactive zero-knowldge authentication.
- Discrete logarithm flavor using exponentiation over groups of prime order.
- Elliptic curve flavor using risettro points via [curve25519-dalek](https://docs.rs/curve25519-dalek/latest/curve25519_dalek/ristretto/index.html).
- Client and server gRPC implementations wrapping the prover and verifier of the protocol, abstracting into register and login workflows.
- The server and demo CLI are available as Docker containers for reproduceable instantiation.

## Project Layout

- [`zkauth`](./zkauth): Core library implementing the [`discrete_logarithm`](./zkauth/src/discrete_logarithm) and [`elliptic_curve`](./zkauth/src/elliptic_curve) flavors of the protocol, with operations defined in `prover.rs` and `verifier.rs` for each.
- [`zkauth-protobuf`](./zkauth-protobuf): Generated protobuf types and stubs for the gRPC service.
- [`zkauth-server`](./zkauth-server): Implementation of the gRPC service as defined in [`zkauth-protobuf/v1.proto`](./zkauth-protobuf/v1.proto), and a binary definition used for execution of the server. This component acts as the verifier in the Chaum-Pedersen protocol.
- [`zkauth-client`](./zkauth-client): Implementation of the gRPC service client as defined in [`zkauth-protobuf/v1.proto`](./zkauth-protobuf/v1.proto). This component acts as the prover in the Chaum-Pedersen protocol.
- [`zkauth-demo-cli`](./zkauth-demo-cli): A simple command-line interface for acting as the client, and hence the prover, to interact with the gRPC service.
- [`tests`](./zkauth): A suite of functional tests that encode the expectations of the client/prover and server/verifier in an end-to-end way.

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
