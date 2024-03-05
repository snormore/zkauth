use dashmap::DashMap;
use moka::sync::Cache;
use std::time::Duration;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use zkauth::{Element, Scalar, Verifier};
use zkauth_protobuf::v1::{
    auth_server::Auth, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, Configuration,
    GetConfigurationRequest, RegisterRequest, RegisterResponse,
};

/// User data for the authentication protocol.
#[derive(Debug)]
struct User {
    y1: Element,
    y2: Element,
}

/// Challenge data for the authentication protocol.
#[derive(Debug, Clone)]
struct Challenge {
    user: String,
    c: Scalar,
    r1: Element,
    r2: Element,
}

/// Session data for the authentication protocol.
#[derive(Debug, Clone, Copy)]
struct Session {
    id: Uuid,
}

/// Service for the authentication protocol.
pub struct Service {
    verifier: Box<dyn Verifier>,
    configuration: Configuration,
    users: DashMap<String, User>,
    challenges: Cache<String, Challenge>,
    sessions: Cache<String, Session>,
}

impl Service {
    /// Creates a new service with the given configuration and verifier.
    pub fn new(configuration: Configuration, verifier: Box<dyn Verifier>) -> Self {
        Self {
            configuration,
            verifier,
            users: DashMap::new(),
            challenges: Cache::builder()
                .time_to_live(Duration::from_secs(300))
                .build(),
            sessions: Cache::builder()
                .time_to_live(Duration::from_secs(3600))
                .build(),
        }
    }
}

/// Implementation of the authentication protocol.
#[tonic::async_trait]
impl Auth for Service {
    /// Returns the configuration for the authentication protocol.
    async fn get_configuration(
        &self,
        _: Request<GetConfigurationRequest>,
    ) -> Result<Response<Configuration>, Status> {
        Ok(Response::new(self.configuration.clone()))
    }

    /// Registers a new user with the given y1 and y2, returning an error if the user is already registered.
    /// # Errors
    /// * Returns an error if the user is already registered.
    /// * Returns an error if the user is empty.
    /// * Returns an error if y1 or y2 is invalid.
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();

        if request.user.is_empty() {
            return Err(Status::invalid_argument("Invalid user argument"));
        }

        let y1: Element = request
            .y1
            .parse()
            .map_err(|_| tonic::Status::invalid_argument("Invalid y1 argument"))?;

        let y2: Element = request
            .y2
            .parse()
            .map_err(|_| tonic::Status::invalid_argument("Invalid y2 argument"))?;

        if self.users.get(&request.user).is_some() {
            return Err(Status::already_exists("User already registered"));
        }

        self.users.insert(request.user, User { y1, y2 });

        Ok(Response::new(RegisterResponse {}))
    }

    /// Creates a new challenge using the given commitment, and returns c in the response along
    /// with the challenge auth id.
    /// # Errors
    /// * Returns an error if the user is not found.
    /// * Returns an error if the user is empty.
    /// * Returns an error if r1 or r2 is invalid.
    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let request = request.into_inner();

        if request.user.is_empty() {
            return Err(Status::invalid_argument("Invalid user argument"));
        }

        let r1: Element = request
            .r1
            .parse()
            .map_err(|_| tonic::Status::invalid_argument("Invalid r1 argument"))?;
        let r2: Element = request
            .r2
            .parse()
            .map_err(|_| tonic::Status::invalid_argument("Invalid r2 argument"))?;

        if self.users.get(&request.user).is_none() {
            return Err(Status::not_found("User not found"));
        }

        // Generate random challenge number c.
        let c = self.verifier.generate_challenge_c();
        log::info!("c = {:?}", c);

        // Store (auth_id, (user, c)) for use in verify_authentication.
        let auth_id = Uuid::new_v4().to_string();
        self.challenges.insert(
            auth_id.clone(),
            Challenge {
                user: request.user,
                c: c.clone(),
                r1,
                r2,
            },
        );

        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id,
            c: c.to_string(),
        }))
    }

    /// Verifies the given s and creates a new session based on it if necessary, returning the
    /// session id in the response.
    /// # Errors
    /// * Returns an error if the challenge is not found.
    /// * Returns an error if the user is not found.
    /// * Returns an error if the auth_id argument is empty.
    /// * Returns an error if s is invalid.
    /// * Returns an error if the verification fails.
    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let request = request.into_inner();

        let s: Scalar = request
            .s
            .parse()
            .map_err(|_| tonic::Status::invalid_argument("Invalid s argument"))?;

        if request.auth_id.is_empty() {
            return Err(Status::invalid_argument("Invalid auth_id argument"));
        }

        let challenge = self
            .challenges
            .get(&request.auth_id)
            .ok_or_else(|| Status::not_found("Challenge not found"))?;

        let user = self
            .users
            .get(&challenge.user)
            .ok_or_else(|| Status::not_found("User not found"))?;

        // Verify and return error if not correct.
        let (r1, r2) = self
            .verifier
            .compute_verification_r1r2(user.y1.clone(), user.y2.clone(), challenge.c, s.clone())
            .map_err(|_| Status::internal("Failed to compute verification r1r2"))?;

        if r1 != challenge.r1 || r2 != challenge.r2 {
            return Err(Status::failed_precondition("Verification failed"));
        }

        let session_key = s.to_string();
        let session = match self.sessions.get(&session_key) {
            None => {
                let session = Session { id: Uuid::new_v4() };
                self.sessions.insert(session_key, session);
                session
            }
            Some(session) => session,
        };

        Ok(Response::new(AuthenticationAnswerResponse {
            session_id: session.id.to_string(),
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use num_traits::One;
    use tonic::Code;
    use tonic::Request;
    use zkauth::discrete_logarithm::{
        configuration::DiscreteLogarithmConfiguration, verifier::DiscreteLogarithmVerifier,
    };

    /// Creates a new service for testing.
    fn test_service() -> Service {
        let config = DiscreteLogarithmConfiguration::generate(16);
        let verifier = Box::new(DiscreteLogarithmVerifier::new(config.clone()));
        Service::new(config.into(), verifier)
    }

    #[cfg(test)]
    mod get_configuration {
        use super::*;

        /// Tests that the get_configuration method returns the correct configuration.
        #[tokio::test]
        async fn succeeds_with_discrete_logarithm_config() -> Result<()> {
            let config = DiscreteLogarithmConfiguration::generate(16);
            let config_pb: Configuration = config.clone().into();
            let verifier = Box::new(DiscreteLogarithmVerifier::new(config));
            let service = Service::new(config_pb.clone(), verifier);

            let resp = service
                .get_configuration(Request::new(GetConfigurationRequest {}))
                .await?
                .into_inner();

            assert_eq!(resp, config_pb);

            Ok(())
        }
    }

    #[cfg(test)]
    mod register {
        use super::*;

        /// Tests that the register method succeeds with valid arguments.
        #[tokio::test]
        async fn succeeds() -> Result<()> {
            let verifier = test_service();
            let resp = verifier
                .register(Request::new(RegisterRequest {
                    user: "peggy".to_string(),
                    y1: "1".to_string(),
                    y2: "1".to_string(),
                }))
                .await?
                .into_inner();

            assert_eq!(resp, RegisterResponse {});

            Ok(())
        }

        /// Tests that the register method returns an error when the user is already registered.
        #[tokio::test]
        async fn returns_error_when_user_already_registered() -> Result<()> {
            let verifier = test_service();
            verifier.users.insert(
                "peggy".to_string(),
                User {
                    y1: One::one(),
                    y2: One::one(),
                },
            );

            let result = verifier
                .register(Request::new(RegisterRequest {
                    user: "peggy".to_string(),
                    y1: "1".to_string(),
                    y2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::AlreadyExists);
            assert_eq!(err.message(), "User already registered");

            Ok(())
        }

        /// Tests that the register method returns an error when the user is empty.
        #[tokio::test]
        async fn returns_error_when_user_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .register(Request::new(RegisterRequest {
                    user: "".to_string(),
                    y1: "1".to_string(),
                    y2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid user argument");

            Ok(())
        }

        /// Tests that the register method returns an error when y1 is empty.
        #[tokio::test]
        async fn returns_error_when_y1_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .register(Request::new(RegisterRequest {
                    user: "peggy".to_string(),
                    y1: "".to_string(),
                    y2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid y1 argument");

            Ok(())
        }

        /// Tests that the register method returns an error when y2 is empty.
        #[tokio::test]
        async fn returns_error_when_y2_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .register(Request::new(RegisterRequest {
                    user: "peggy".to_string(),
                    y1: "1".to_string(),
                    y2: "".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid y2 argument");

            Ok(())
        }

        /// Tests that the register method returns an error when y1 is not a number.
        #[tokio::test]
        async fn returns_error_when_y1_is_not_a_number() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .register(Request::new(RegisterRequest {
                    user: "peggy".to_string(),
                    y1: "not-a-number".to_string(),
                    y2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid y1 argument");

            Ok(())
        }

        /// Tests that the register method returns an error when y2 is not a number.
        #[tokio::test]
        async fn returns_error_when_y2_is_not_a_number() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .register(Request::new(RegisterRequest {
                    user: "peggy".to_string(),
                    y1: "1".to_string(),
                    y2: "not-a-number".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid y2 argument");

            Ok(())
        }
    }

    #[cfg(test)]
    mod create_authentication_challenge {
        use super::*;

        /// Tests that the create_authentication_challenge method succeeds with valid arguments.
        #[tokio::test]
        async fn succeeds() -> Result<()> {
            let verifier = test_service();
            verifier.users.insert(
                "peggy".to_string(),
                User {
                    y1: One::one(),
                    y2: One::one(),
                },
            );

            let resp = verifier
                .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                    user: "peggy".to_string(),
                    r1: "1".to_string(),
                    r2: "1".to_string(),
                }))
                .await?
                .into_inner();

            Uuid::parse_str(&resp.auth_id)?;
            resp.c.parse::<Scalar>().unwrap();

            Ok(())
        }

        /// Tests that the create_authentication_challenge method returns an error when the user is
        #[tokio::test]
        async fn returns_not_found_when_unknown_user() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                    user: "unknown".to_string(),
                    r1: "1".to_string(),
                    r2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::NotFound);
            assert_eq!(err.message(), "User not found");

            Ok(())
        }

        /// Tests that the create_authentication_challenge method returns an error when the user is empty.
        #[tokio::test]
        async fn returns_error_when_user_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                    user: "".to_string(),
                    r1: "1".to_string(),
                    r2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid user argument");

            Ok(())
        }

        /// Tests that the create_authentication_challenge method returns an error when r1 is empty.
        #[tokio::test]
        async fn returns_error_when_r1_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                    user: "peggy".to_string(),
                    r1: "".to_string(),
                    r2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid r1 argument");

            Ok(())
        }

        /// Tests that the create_authentication_challenge method returns an error when r2 is empty.
        #[tokio::test]
        async fn returns_error_when_r2_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                    user: "peggy".to_string(),
                    r1: "1".to_string(),
                    r2: "".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid r2 argument");

            Ok(())
        }

        /// Tests that the create_authentication_challenge method returns an error when r1 is not a number.
        #[tokio::test]
        async fn returns_error_when_r1_is_not_a_number() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                    user: "peggy".to_string(),
                    r1: "not-a-number".to_string(),
                    r2: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid r1 argument");

            Ok(())
        }

        /// Tests that the create_authentication_challenge method returns an error when r2 is not a number.
        #[tokio::test]
        async fn returns_error_when_r2_is_not_a_number() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .create_authentication_challenge(Request::new(AuthenticationChallengeRequest {
                    user: "peggy".to_string(),
                    r1: "1".to_string(),
                    r2: "not-a-number".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid r2 argument");

            Ok(())
        }
    }

    #[cfg(test)]
    mod verify_authentication {
        use super::*;
        use zkauth::{discrete_logarithm::prover::DiscreteLogarithmProver, Prover};

        /// Tests that the verify_authentication method succeeds with valid arguments.
        #[tokio::test]
        async fn succeeds() -> Result<()> {
            let config = DiscreteLogarithmConfiguration::generate(16);
            let config_pb: Configuration = config.clone().into();
            let verifier = DiscreteLogarithmVerifier::new(config.clone());
            let service = Service::new(config_pb.clone(), Box::new(verifier.clone()));

            let prover = DiscreteLogarithmProver::new(config);

            let user = "peggy".to_string();
            let auth_id = Uuid::new_v4().to_string();

            let x = prover.generate_registration_x();
            let (y1, y2) = prover.compute_registration_y1y2(x.clone())?;
            let k = prover.generate_challenge_k();
            let (r1, r2) = prover.compute_challenge_commitment_r1r2(k.clone()).unwrap();
            let c = verifier.generate_challenge_c();
            let s = prover
                .compute_challenge_response_s(x, k, c.clone())
                .unwrap();

            service.users.insert(user.clone(), User { y1, y2 });
            service
                .challenges
                .insert(auth_id.clone(), Challenge { user, c, r1, r2 });

            let resp = service
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id: auth_id.clone(),
                    s: s.to_string(),
                }))
                .await?
                .into_inner();

            Uuid::parse_str(&resp.session_id)?;
            let original_session_id = resp.session_id;

            let resp = service
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id,
                    s: s.to_string(),
                }))
                .await?
                .into_inner();

            assert_eq!(resp.session_id, original_session_id);

            Ok(())
        }

        /// Tests that the verify_authentication method returns an error when the challenge is not found.
        #[tokio::test]
        async fn returns_not_found_when_unknown_challenge() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id: "unknown".to_string(),
                    s: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::NotFound);
            assert_eq!(err.message(), "Challenge not found");

            Ok(())
        }

        /// Tests that the verify_authentication method returns an error when the user is not found.
        #[tokio::test]
        async fn returns_not_found_when_unknown_user() -> Result<()> {
            let verifier = test_service();
            verifier.challenges.insert(
                "id".to_string(),
                Challenge {
                    user: "unknown".to_string(),
                    c: One::one(),
                    r1: One::one(),
                    r2: One::one(),
                },
            );

            let result = verifier
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id: "id".to_string(),
                    s: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::NotFound);
            assert_eq!(err.message(), "User not found");

            Ok(())
        }

        /// Tests that the verify_authentication method returns an error when the auth_id is empty.
        #[tokio::test]
        async fn returns_error_when_auth_id_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id: "".to_string(),
                    s: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid auth_id argument");

            Ok(())
        }

        /// Tests that the verify_authentication method returns an error when r1 is empty.
        #[tokio::test]
        async fn returns_error_when_r1_is_empty() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id: "".to_string(),
                    s: "".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid s argument");

            Ok(())
        }

        /// Tests that the verify_authentication method returns an error when r1 is not a number.
        #[tokio::test]
        async fn returns_error_when_r1_is_not_a_number() -> Result<()> {
            let verifier = test_service();
            let result = verifier
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id: "".to_string(),
                    s: "not-a-number".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::InvalidArgument);
            assert_eq!(err.message(), "Invalid s argument");

            Ok(())
        }

        /// Tests that the verify_authentication method returns an error when the verification fails.
        #[tokio::test]
        async fn returns_verification_failed() -> Result<()> {
            let verifier = test_service();

            verifier.users.insert(
                "peggy".to_string(),
                User {
                    y1: One::one(),
                    y2: One::one(),
                },
            );
            verifier.challenges.insert(
                "id".to_string(),
                Challenge {
                    user: "peggy".to_string(),
                    c: One::one(),
                    r1: One::one(),
                    r2: One::one(),
                },
            );

            let result = verifier
                .verify_authentication(Request::new(AuthenticationAnswerRequest {
                    auth_id: "id".to_string(),
                    s: "1".to_string(),
                }))
                .await;

            let err = result.unwrap_err();
            assert_eq!(err.code(), Code::FailedPrecondition);
            assert_eq!(err.message(), "Verification failed");

            Ok(())
        }
    }
}
