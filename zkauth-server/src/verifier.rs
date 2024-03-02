use dashmap::DashMap;
use moka::sync::Cache;
use num_bigint::{BigInt, BigUint, RandomBits};
use num_traits::One;
use rand::Rng;
use std::time::Duration;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use zkauth_pb::v1::{
    auth_server::Auth, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, GetPublicParametersRequest,
    GetPublicParametersResponse, RegisterRequest, RegisterResponse,
};

use crate::parameters::{default_parameters, generate_parameters, Parameters};

#[derive(Debug)]
struct User {
    y1: BigInt,
    y2: BigInt,
}

#[derive(Debug, Clone)]
struct Challenge {
    user: String,
    c: BigUint,
    r1: BigInt,
    r2: BigInt,
}

#[derive(Debug, Clone, Copy)]
struct Session {
    id: Uuid,
}

pub struct Verifier {
    parameters: Parameters,
    users: DashMap<String, User>,
    challenges: Cache<String, Challenge>,
    sessions: Cache<String, Session>,
}

impl Verifier {
    pub fn new(parameters: Parameters) -> Self {
        Self {
            parameters,
            users: DashMap::new(),
            challenges: Cache::builder()
                .time_to_live(Duration::from_secs(300))
                .build(),
            sessions: Cache::builder()
                .time_to_live(Duration::from_secs(3600))
                .build(),
        }
    }

    pub fn generated(prime_bits: usize) -> Self {
        Self::new(generate_parameters(prime_bits))
    }
}

impl Default for Verifier {
    fn default() -> Self {
        Self::new(default_parameters())
    }
}

#[tonic::async_trait]
impl Auth for Verifier {
    async fn get_public_parameters(
        &self,
        _: Request<GetPublicParametersRequest>,
    ) -> Result<Response<GetPublicParametersResponse>, Status> {
        Ok(Response::new(GetPublicParametersResponse {
            p: self.parameters.p.to_string(),
            q: self.parameters.q.to_string(),
            g: self.parameters.g.to_string(),
            h: self.parameters.h.to_string(),
        }))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();

        if request.user.is_empty() {
            return Err(Status::invalid_argument("Invalid user argument"));
        }

        let y1 = request
            .y1
            .parse::<BigInt>()
            .map_err(|_| tonic::Status::invalid_argument("Invalid y1 argument"))?;

        let y2 = request
            .y2
            .parse::<BigInt>()
            .map_err(|_| tonic::Status::invalid_argument("Invalid y2 argument"))?;

        if self.users.get(&request.user).is_some() {
            return Err(Status::already_exists("User already registered"));
        }

        self.users.insert(request.user, User { y1, y2 });

        Ok(Response::new(RegisterResponse {}))
    }

    /// Creates a new challenge using the given commitment, and returns c in the response along
    /// with the challenge auth id.
    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        let request = request.into_inner();

        if request.user.is_empty() {
            return Err(Status::invalid_argument("Invalid user argument"));
        }

        let r1 = request
            .r1
            .parse::<BigInt>()
            .map_err(|_| tonic::Status::invalid_argument("Invalid r1 argument"))?;
        let r2 = request
            .r2
            .parse::<BigInt>()
            .map_err(|_| tonic::Status::invalid_argument("Invalid r2 argument"))?;

        if self.users.get(&request.user).is_none() {
            return Err(Status::not_found("User not found"));
        }

        // Generate random challenge number c.
        // Should not be negative because it's used as an exponent.
        let mut rng = rand::thread_rng();
        let c: BigUint = rng.sample(RandomBits::new(32));

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
    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        let request = request.into_inner();

        let s = request
            .s
            .parse::<BigInt>()
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
        let p = self.parameters.p.clone();
        let one = One::one();
        let c: BigInt = challenge.c.clone().into();
        let r1 = (self.parameters.g.modpow(&s, &p) * user.y1.modpow(&c, &p)).modpow(&one, &p);
        let r2 = (self.parameters.h.modpow(&s, &p) * user.y2.modpow(&c, &p)).modpow(&one, &p);

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
        log::info!("Session: {}", session.id);

        Ok(Response::new(AuthenticationAnswerResponse {
            session_id: session.id.to_string(),
        }))
    }
}

#[cfg(test)]
mod get_public_parameters {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let verifier = Verifier::default();
        let resp = verifier
            .get_public_parameters(Request::new(GetPublicParametersRequest {}))
            .await?
            .into_inner();

        resp.p.parse::<BigInt>()?;
        resp.q.parse::<BigInt>()?;
        resp.g.parse::<BigInt>()?;
        resp.h.parse::<BigInt>()?;

        Ok(())
    }
}

#[cfg(test)]
mod register {
    use super::*;
    use anyhow::Result;
    use tonic::Code;

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_user_already_registered() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_user_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_y1_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_y2_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_y1_is_not_a_number() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_y2_is_not_a_number() -> Result<()> {
        let verifier = Verifier::default();
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
    use anyhow::Result;
    use tonic::Code;

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let verifier = Verifier::default();
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
        resp.c.parse::<BigInt>()?;

        Ok(())
    }

    #[tokio::test]
    async fn returns_not_found_when_unknown_user() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_user_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_r1_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_r2_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_r1_is_not_a_number() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_r2_is_not_a_number() -> Result<()> {
        let verifier = Verifier::default();
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
    use anyhow::Result;
    use num_traits::Zero;
    use tonic::Code;

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let verifier = Verifier::default();

        let params = verifier
            .get_public_parameters(Request::new(GetPublicParametersRequest {}))
            .await?
            .into_inner();
        let p = params.p.parse::<BigInt>().unwrap();
        let q = params.q.parse::<BigInt>().unwrap();
        let g = params.g.parse::<BigInt>().unwrap();
        let h = params.h.parse::<BigInt>().unwrap();

        let mut rng = rand::thread_rng();

        let x: BigUint = rng.sample(RandomBits::new(256));
        let signed_x: BigInt = x.clone().into();
        let y1 = g.modpow(&signed_x, &p);
        let y2 = h.modpow(&signed_x, &p);

        let c: BigUint = rng.sample(RandomBits::new(32));
        let signed_c: BigInt = c.clone().into();
        let k: BigUint = rng.sample(RandomBits::new(32));
        let signed_k: BigInt = k.clone().into();
        let r1 = g.modpow(&signed_k, &p);
        let r2 = h.modpow(&signed_k, &p);

        let mut s = (signed_k - signed_c * signed_x) % &q;
        if s < Zero::zero() {
            s += q;
        }

        let user = "peggy".to_string();
        let auth_id = Uuid::new_v4().to_string();

        verifier.users.insert(user.clone(), User { y1, y2 });
        verifier
            .challenges
            .insert(auth_id.clone(), Challenge { user, c, r1, r2 });

        let resp = verifier
            .verify_authentication(Request::new(AuthenticationAnswerRequest {
                auth_id: auth_id.clone(),
                s: s.to_string(),
            }))
            .await?
            .into_inner();

        Uuid::parse_str(&resp.session_id)?;
        let original_session_id = resp.session_id;

        let resp = verifier
            .verify_authentication(Request::new(AuthenticationAnswerRequest {
                auth_id,
                s: s.to_string(),
            }))
            .await?
            .into_inner();

        assert_eq!(resp.session_id, original_session_id);

        Ok(())
    }

    #[tokio::test]
    async fn returns_not_found_when_unknown_challenge() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_not_found_when_unknown_user() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_auth_id_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_r1_is_empty() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_error_when_r1_is_not_a_number() -> Result<()> {
        let verifier = Verifier::default();
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

    #[tokio::test]
    async fn returns_verification_failed() -> Result<()> {
        let verifier = Verifier::default();

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
