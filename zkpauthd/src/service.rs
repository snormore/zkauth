use dashmap::DashMap;
use num_bigint::{BigInt, BigUint, RandomBits, ToBigInt};
use num_traits::One;
use rand::Rng;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use zkpauthpb::v1::{
    auth_server::Auth, AuthenticationAnswerRequest, AuthenticationAnswerResponse,
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, GetPublicParametersRequest,
    GetPublicParametersResponse, RegisterRequest, RegisterResponse,
};

const RANDOM_NONCE_LENGTH_BITS: u64 = 32;

#[derive(Debug)]
struct Parameters {
    p: BigInt,
    q: BigInt,
    g: BigInt,
    h: BigInt,
}

#[derive(Debug)]
struct Registration {
    y1: BigInt,
    y2: BigInt,
}

#[derive(Debug)]
struct Login {
    user: String,
    c: BigUint,
    r1: BigInt,
    r2: BigInt,
}

pub struct Service {
    parameters: Parameters,
    registrations: DashMap<String, Registration>,
    logins: DashMap<String, Login>,
    sessions: DashMap<String, ()>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            parameters: Parameters {
                // Values from https://github.com/twilker/cp-zkp/blob/main/src/lib/chaum_pedersen/algorithm.rs#L11-L15
                // TODO: Support generating random similar to https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/utils.py
                p: "42765216643065397982265462252423826320512529931694366715111734768493812630447"
                    .parse::<BigInt>()
                    .unwrap(),
                q: "21382608321532698991132731126211913160256264965847183357555867384246906315223"
                    .parse::<BigInt>()
                    .unwrap(),
                g: 4.to_bigint().unwrap(),
                h: 9.to_bigint().unwrap(),
            },
            registrations: DashMap::new(),
            logins: DashMap::new(),
            sessions: DashMap::new(),
        }
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
        // TODO: consider making this an "into" method on Parameters instead
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

        log::info!("{:?}", request);

        // TODO: handle case where user already registered, if necessary

        // Store (user, (y1, y2)) for use in create_authentication_challenge and verify_authentication
        self.registrations.insert(
            request.user,
            Registration {
                y1: request.y1.parse::<BigInt>().unwrap(),
                y2: request.y2.parse::<BigInt>().unwrap(),
            },
        );

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        // https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/chaum_pedersen/verifier.py#L25
        let request = request.into_inner();
        let r1 = request.r1.parse::<BigInt>().unwrap();
        let r2 = request.r2.parse::<BigInt>().unwrap();

        log::info!("{:?}", request);

        let mut rng = rand::thread_rng();

        // Generate random challenge number c.
        // Should not be negative because it's used as an exponent.
        let c: BigUint = rng.sample(RandomBits::new(RANDOM_NONCE_LENGTH_BITS));

        let auth_id = Uuid::new_v4().to_string();

        // TODO: check that user is registered, otherwise return error

        // Store (auth_id, (user, c)) for use in verify_authentication
        self.logins.insert(
            auth_id.clone(),
            Login {
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

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        // https://github.com/neongazer/zkp-auth-py/blob/main/zkp_auth/sigma_protocols/chaum_pedersen/verifier.py#L43-L46
        // https://github.com/kobby-pentangeli/chaum-pedersen-zkp/blob/master/src/lib.rs#L72-L80
        let request = request.into_inner();
        let s = request.s.parse::<BigInt>().unwrap();

        log::info!("{:?}", request);

        // Lookup (auth_id, (user, c))
        // TODO: handle error / not found
        let login = self.logins.get(&request.auth_id).unwrap();
        log::info!("{:?}", login.value());

        // Lookup (user, (y1, y2))
        // TODO: handle error / not found
        let registration = self.registrations.get(&login.user).unwrap();
        log::info!("{:?}", registration.value());

        // TODO: verify and return error if not correct
        let p = self.parameters.p.clone();
        let one = One::one();
        let c: BigInt = login.c.clone().into();
        let r1 =
            (self.parameters.g.modpow(&s, &p) * registration.y1.modpow(&c, &p)).modpow(&one, &p);
        let r2 =
            (self.parameters.h.modpow(&s, &p) * registration.y2.modpow(&c, &p)).modpow(&one, &p);

        // log::info!("condition1: {} == {}", r1, login.r1);
        // log::info!("condition2: {} == {}", r2, login.r2);

        if r1 != login.r1 || r2 != login.r2 {
            return Err(Status::failed_precondition("verification failed"));
        }

        // TODO: if session already exists, then return that session id instead of a new one

        let session_id = Uuid::new_v4().to_string();
        self.sessions.insert(session_id.clone(), ());
        log::info!("Session: {}", session_id);

        Ok(Response::new(AuthenticationAnswerResponse { session_id }))
    }
}
