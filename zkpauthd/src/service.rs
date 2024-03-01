use num_bigint::{BigInt, ToBigInt};
use tonic::{Request, Response, Status};
use zkpauthpb::v1::{
    auth_server::Auth, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, GetPublicParametersRequest,
    GetPublicParametersResponse, RegisterRequest, RegisterResponse,
};

pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl Auth for Service {
    async fn get_public_parameters(
        &self,
        _: Request<GetPublicParametersRequest>,
    ) -> Result<Response<GetPublicParametersResponse>, Status> {
        Ok(Response::new(GetPublicParametersResponse {
            // Values from https://github.com/twilker/cp-zkp/blob/main/src/lib/chaum_pedersen/algorithm.rs#L11-L15
            // TODO: Support generating random similar to https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/utils.py
            p: "42765216643065397982265462252423826320512529931694366715111734768493812630447"
                .parse::<BigInt>()
                .unwrap()
                .to_bytes_be()
                .1
                .into(),
            q: "21382608321532698991132731126211913160256264965847183357555867384246906315223"
                .parse::<BigInt>()
                .unwrap()
                .to_bytes_be()
                .1
                .into(),
            g: 4.to_bigint().unwrap().to_bytes_be().1.into(),
            h: 9.to_bigint().unwrap().to_bytes_be().1.into(),
            bit_size: 256,
        }))
    }

    async fn register(
        &self,
        _: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        todo!()
    }

    async fn create_authentication_challenge(
        &self,
        _: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!()
    }

    async fn verify_authentication(
        &self,
        _: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        todo!()
    }
}
