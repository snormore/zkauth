# zkauth

This is an implementation of the Chaum-Pedersen ZKP protocol which allows users to register and login with a server without providing their passwords.

[![codecov](https://codecov.io/gh/snormore/zkauth/graph/badge.svg?token=JN2KIKA175)](https://codecov.io/gh/snormore/zkauth)

The Chaum-Pedersen ZKP protocol is explained in [Cryptography: An Introduction](https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf), chapter 25, section 3.2.

# TODO

- Support elliptical instead of scalar https://github.com/gagiuntoli/zkp_chaum_pedersen/blob/7b0c77adf547cef2df17b42ecf49780c679d3e9a/src/lib.rs#L165
- Docker/compose for the setup
- Deploy to AWS
- Add bench tests

- Fill out comment docs for deployed rust docs
- Fill out README
- Use external kv store for high availability of multiple replicas of the server
- Add demo web app using zkauth-client
- Add devcontainer
- CI pipeline for test/lint/build/release https://github.com/BamPeers/rust-ci-github-actions-workflow
- CI pipeline for docker image https://github.com/snormore/mds/blob/main/.github/workflows/ci-image.yml
- Release package to crates.io
- Fill out remaining test coverage for main/bins
