use anyhow::Result;
use tonic::{
    transport::{Channel, Endpoint, Server, Uri},
    Request, Response, Status,
};
use tower::service_fn;
use zkauth_protobuf::v1::{
    auth_client::AuthClient,
    auth_server::{Auth, AuthServer},
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, Configuration, GetConfigurationRequest, RegisterRequest,
    RegisterResponse,
};

pub async fn mock_client() -> Result<AuthClient<Channel>> {
    let (client, server) = tokio::io::duplex(1024);

    let verifier = MockVerifier::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(AuthServer::new(verifier))
            .serve_with_incoming(tokio_stream::once(Ok::<_, std::io::Error>(server)))
            .await
    });

    // Move client to an option so we can _move_ the inner value
    // on the first attempt to connect. All other attempts will fail.
    let mut client = Some(client);
    let channel = Endpoint::try_from("http://[::]:0")?
        .connect_with_connector(service_fn(move |_: Uri| {
            let client = client.take();

            async move {
                client.ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Client already taken")
                })
            }
        }))
        .await?;

    let client = AuthClient::new(channel);

    Ok(client)
}

#[derive(Default)]
pub struct MockVerifier {}

#[tonic::async_trait]
impl Auth for MockVerifier {
    async fn get_configuration(
        &self,
        _: Request<GetConfigurationRequest>,
    ) -> Result<Response<Configuration>, Status> {
        Ok(Response::new(Configuration {
            flavor: Some(
                zkauth_protobuf::v1::configuration::Flavor::DiscreteLogarithm(
                    zkauth_protobuf::v1::configuration::DiscreteLogarithm {
                        p: "1".to_string(),
                        q: "1".to_string(),
                        g: "1".to_string(),
                        h: "1".to_string(),
                    },
                ),
            ),
        }))
    }

    async fn register(
        &self,
        _: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        _: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id: "auth-id".to_string(),
            c: "1".to_string(),
        }))
    }

    async fn verify_authentication(
        &self,
        _: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        Ok(Response::new(AuthenticationAnswerResponse {
            session_id: "session-id".to_string(),
        }))
    }
}
