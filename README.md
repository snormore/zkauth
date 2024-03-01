# zkp-auth

ZKP protocol for authentication

# TODO

- `brew install buf`
- `( cd zkpauthpb && scripts/generate )`
- Clean up zkpauthpb crate generation and features
- Better error handling whenever )? or await? or unwrap() is used so server doesn't get killed from a panic
- Add bench tests
- Docker/compose for the setup
- Use external kv store for high availability of multiple replicas of the server
- Deploy to AWS
- Support elliptical instead of scalar

## Places it can be extended

- Use an external/shared datastore for user challenges so that the server can be deployed with multiple replicas to be highly available
