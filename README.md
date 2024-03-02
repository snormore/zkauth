# zkp-auth

ZKP protocol for authentication

# TODO

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
