use tonic::{Request, Response, Status};
use zkpauthpb::v1::{
    auth_server::Auth, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, RegisterRequest,
    RegisterResponse,
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
