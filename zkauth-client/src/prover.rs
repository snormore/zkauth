use num_bigint::{BigInt, BigUint, RandomBits};
use num_traits::Zero;
use rand::Rng;
use sha2::{Digest, Sha256};
use tonic::{transport::Channel, Status};
use zkauth_pb::v1::{
    auth_client::AuthClient, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
    GetPublicParametersRequest, RegisterRequest,
};

#[derive(Debug)]
pub struct Prover {
    client: AuthClient<Channel>,
    parameters: Parameters,
    user: String,
    x: BigUint,
}

#[derive(Debug, PartialEq)]
struct Parameters {
    p: BigInt,
    q: BigInt,
    g: BigInt,
    h: BigInt,
}

impl Prover {
    pub async fn new(
        mut client: AuthClient<Channel>,
        user: String,
        password: String,
    ) -> Result<Self, Status> {
        if user.is_empty() {
            return Err(Status::invalid_argument("Invalid user argument"));
        }

        if password.is_empty() {
            return Err(Status::invalid_argument("Invalid password argument"));
        }

        let params = client
            .get_public_parameters(GetPublicParametersRequest {})
            .await?
            .into_inner();
        let p = params.p.parse::<BigInt>().unwrap();
        let q = params.q.parse::<BigInt>().unwrap();
        let g = params.g.parse::<BigInt>().unwrap();
        let h = params.h.parse::<BigInt>().unwrap();

        // Generate random secret number x.
        // Should not be negative because it's used as an exponent.
        // let mut rng = rand::thread_rng();
        // let x: BigUint = rng.sample(RandomBits::new(RANDOM_SECRET_LENGTH_BITS));

        // Convert password string to x number.
        let x = BigUint::from_bytes_be(&Sha256::digest(password.as_bytes()));

        Ok(Prover {
            client,
            parameters: Parameters { p, q, g, h },
            user,
            x,
        })
    }

    pub async fn register(&self) -> Result<(), Status> {
        let p = &self.parameters.p;
        let g = &self.parameters.g;
        let h = &self.parameters.h;

        // Compute y1 and y2 for registration.
        let signed_x: BigInt = self.x.clone().into();
        let y1 = g.modpow(&signed_x, p);
        let y2 = h.modpow(&signed_x, p);
        log::debug!("y1 = {}", y1);
        log::debug!("y2 = {}", y2);

        // Send register request.
        let resp = self
            .client
            .clone()
            .register(RegisterRequest {
                user: self.user.clone(),
                y1: y1.to_string(),
                y2: y2.to_string(),
            })
            .await?
            .into_inner();

        log::debug!("{:?}", resp);

        Ok(())
    }

    pub async fn login(&self) -> Result<(), Status> {
        // Generate random number k.
        // Should not be negative because it's used as an exponent.
        let mut rng = rand::thread_rng();
        let k: BigUint = rng.sample(RandomBits::new(32));
        let signed_k: BigInt = k.clone().into();

        let p = &self.parameters.p;
        let q = &self.parameters.q;
        let g = &self.parameters.g;
        let h = &self.parameters.h;

        // Compute commitment (r1, r2) for authentication challenge.
        let r1 = g.modpow(&signed_k, p);
        let r2 = h.modpow(&signed_k, p);
        log::debug!("r1 = {}", r1);
        log::debug!("r2 = {}", r2);

        // Send create_authentication_challenge request.
        let resp = self
            .client
            .clone()
            .create_authentication_challenge(AuthenticationChallengeRequest {
                user: self.user.clone(),
                r1: r1.to_string(),
                r2: r2.to_string(),
            })
            .await?
            .into_inner();
        let c = resp.c.parse::<BigInt>().unwrap();

        log::debug!("{:?}", resp);

        // Compute challenge response s.
        // Should not be negative because it's used as an exponent.
        let signed_x: BigInt = self.x.clone().into();
        let mut s = (signed_k - c * signed_x) % q;
        if s < Zero::zero() {
            s += q;
        }
        log::debug!("s = {}", s);

        // Send verify_authentication request.
        let resp = self
            .client
            .clone()
            .verify_authentication(AuthenticationAnswerRequest {
                auth_id: resp.auth_id,
                s: s.to_string(),
            })
            .await?
            .into_inner();

        log::debug!("{:?}", resp);

        Ok(())
    }
}

#[cfg(test)]
mod new {
    use super::*;
    use crate::test::mock_client;
    use anyhow::Result;
    use num_traits::One;

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let client = mock_client().await?;
        let prover = Prover::new(client, "user".to_string(), "password".to_string())
            .await
            .unwrap();

        assert_eq!(
            prover.parameters,
            Parameters {
                p: One::one(),
                q: One::one(),
                g: One::one(),
                h: One::one(),
            }
        );

        Ok(())
    }
}

#[cfg(test)]
mod register {
    use super::*;
    use crate::test::mock_client;
    use anyhow::Result;

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let client = mock_client().await?;
        let prover = Prover::new(client, "user".to_string(), "password".to_string())
            .await
            .unwrap();

        prover.register().await.unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod login {
    use super::*;
    use crate::test::mock_client;
    use anyhow::Result;

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let client = mock_client().await?;
        let prover = Prover::new(client, "user".to_string(), "password".to_string())
            .await
            .unwrap();

        prover.login().await.unwrap();

        Ok(())
    }
}
