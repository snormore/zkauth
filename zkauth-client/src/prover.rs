use num_bigint::{BigInt, BigUint, RandomBits};
use num_traits::Zero;
use rand::rngs::ThreadRng;
use rand::Rng;
use sha2::{Digest, Sha256};
use tonic::transport::Channel;
use zkauth_proto::v1::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    GetPublicParametersRequest, RegisterRequest,
};

pub struct Prover {
    client: AuthClient<Channel>,
    parameters: Parameters,
    rng: ThreadRng,
    user: String,
    x: BigUint,
}

#[derive(Debug)]
struct Parameters {
    p: BigInt,
    q: BigInt,
    g: BigInt,
    h: BigInt,
}

impl Prover {
    pub async fn new(
        address: String,
        user: String,
        password: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client = AuthClient::connect(address).await?;

        let params = client
            .get_public_parameters(GetPublicParametersRequest {})
            .await?
            .into_inner();
        let p = params.p.parse::<BigInt>().unwrap();
        let q = params.q.parse::<BigInt>().unwrap();
        let g = params.g.parse::<BigInt>().unwrap();
        let h = params.h.parse::<BigInt>().unwrap();

        let rng = rand::thread_rng();

        // Generate random secret number x.
        // Should not be negative because it's used as an exponent.
        // let x: BigUint = rng.sample(RandomBits::new(RANDOM_SECRET_LENGTH_BITS));

        // Convert password string to x number.
        let x = BigUint::from_bytes_be(&Sha256::digest(password.as_bytes()));

        Ok(Prover {
            client,
            parameters: Parameters { p, q, g, h },
            rng,
            user,
            x,
        })
    }

    pub async fn register(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let p = &self.parameters.p;
        let g = &self.parameters.g;
        let h = &self.parameters.h;

        // Compute y1 and y2 for registration.
        let signed_x: BigInt = self.x.clone().into();
        let y1 = g.modpow(&signed_x, p);
        let y2 = h.modpow(&signed_x, p);
        log::info!("y1 = {}", y1);
        log::info!("y2 = {}", y2);

        // Send register request.
        let resp = self
            .client
            .register(RegisterRequest {
                user: self.user.clone(),
                y1: y1.to_string(),
                y2: y2.to_string(),
            })
            .await?
            .into_inner();

        log::info!("{:?}", resp);

        Ok(())
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Generate random number k.
        // Should not be negative because it's used as an exponent.
        let k: BigUint = self.rng.sample(RandomBits::new(32));
        let signed_k: BigInt = k.clone().into();

        let p = &self.parameters.p;
        let q = &self.parameters.q;
        let g = &self.parameters.g;
        let h = &self.parameters.h;

        // Compute commitment (r1, r2) for authentication challenge.
        let r1 = g.modpow(&signed_k, p);
        let r2 = h.modpow(&signed_k, p);
        log::info!("r1 = {}", r1);
        log::info!("r2 = {}", r2);

        // Send create_authentication_challenge request.
        let resp = self
            .client
            .create_authentication_challenge(AuthenticationChallengeRequest {
                user: self.user.clone(),
                r1: r1.to_string(),
                r2: r2.to_string(),
            })
            .await?
            .into_inner();
        let c = resp.c.parse::<BigInt>().unwrap();

        log::info!("{:?}", resp);

        // Compute challenge response s.
        // Should not be negative because it's used as an exponent.
        let signed_x: BigInt = self.x.clone().into();
        let mut s = (signed_k - c * signed_x) % q;
        if s < Zero::zero() {
            s += q;
        }
        log::info!("s = {}", s);

        // Send verify_authentication request.
        let resp = self
            .client
            .verify_authentication(AuthenticationAnswerRequest {
                auth_id: resp.auth_id,
                s: s.to_string(),
            })
            .await?
            .into_inner();

        log::info!("{:?}", resp);

        Ok(())
    }
}