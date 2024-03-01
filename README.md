# zkp-auth

ZKP protocol for authentication

# TODO

- `brew install buf`
- `( cd zkpauthpb && scripts/generate )`
- Clean up zkpauthpb crate generation and features
- Better error handling whenever )? or await? is used so server doesn't get killed from a panic

## Places it can be extended

- Use an external/shared datastore for user challenges so that the server can be deployed with multiple replicas to be highly available
-
