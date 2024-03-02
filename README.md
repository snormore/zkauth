# zkp-auth

ZKP protocol for authentication

# TODO

- `brew install buf`
- `( cd zkpauthpb && scripts/generate )`
- Finish off prover-side test coverage
- Better error handling whenever )? or await? or unwrap() is used so server doesn't get killed from a panic
- Add bench tests
- Support elliptical instead of scalar
- Fill out README
- Generate rust docs and deploy to GH pages
- Docker/compose for the setup
- Use external kv store for high availability of multiple replicas of the server
- Deploy to AWS

## Places it can be extended

- Use an external/shared datastore for user challenges so that the server can be deployed with multiple replicas to be highly available
