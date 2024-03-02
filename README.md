# zkauth

This is an implementation of the Chaum-Pedersen ZKP protocol which allows users to register and login with a server without providing their passwords.

[![codecov](https://codecov.io/gh/snormore/zkauth/graph/badge.svg?token=JN2KIKA175)](https://codecov.io/gh/snormore/zkauth)

The protocol is explained in [Cryptography: An Introduction](https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf), chapter 25, section 3.2.

# TODO

- Add github workflows for codecov, docs on gh pages, and ci-image
- Add devcontainer
- Finish off prover-side test coverage
- Add integration tests
- Add tests for multiple logins/challenges concurrently for the same user
- Support elliptical instead of scalar https://github.com/gagiuntoli/zkp_chaum_pedersen/blob/7b0c77adf547cef2df17b42ecf49780c679d3e9a/src/lib.rs#L165
- Docker/compose for the setup
- Deploy to AWS
- Generate rust docs and deploy to GH pages, fill out comment docs
- Fill out README
- Add bench tests
- Use external kv store for high availability of multiple replicas of the server
- Fix any remaining TODOs in comments
- Add demo web app using zkauth-client
