use bytes::Bytes;
use curve25519_dalek::{ristretto::CompressedRistretto, RistrettoPoint};
use num_bigint::{BigInt, Sign};
use sha2::{Digest, Sha256};
use tonic::{transport::Channel, Status};
use zkauth::{
    discrete_logarithm::prover::DiscreteLogarithmProver,
    elliptic_curve::prover::EllipticCurveProver, Prover,
};
use zkauth_pb::v1::{
    auth_client::AuthClient, configuration::Operations, AuthenticationAnswerRequest,
    AuthenticationChallengeRequest, GetConfigurationRequest, RegisterRequest,
};

#[derive(Debug)]
pub struct Client {
    client: AuthClient<Channel>,
    prover: Box<dyn Prover>,
    user: String,
    x: Bytes,
}

impl Client {
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

        let config = client
            .get_configuration(GetConfigurationRequest {})
            .await?
            .into_inner();
        let prover: Box<dyn Prover> = match config.operations {
            Some(Operations::DiscreteLogarithm(config)) => Box::new(DiscreteLogarithmProver::new(
                bytes_to_bigint(config.p),
                bytes_to_bigint(config.q),
                bytes_to_bigint(config.g),
                bytes_to_bigint(config.h),
            )),
            Some(Operations::EllipticCurve(config)) => Box::new(EllipticCurveProver::new(
                bytes_to_ristretto_point(config.g),
                bytes_to_ristretto_point(config.h),
            )),
            None => return Err(Status::internal("unknown server configuration")),
        };
        // let p = params.p.parse::<BigInt>().unwrap();
        // let q = params.q.parse::<BigInt>().unwrap();
        // let g = params.g.parse::<BigInt>().unwrap();
        // let h = params.h.parse::<BigInt>().unwrap();
        // let prover = DiscreteLogarithmProver::new();

        // Generate random secret number x.
        // Should not be negative because it's used as an exponent.
        // let mut rng = rand::thread_rng();
        // let x: BigUint = rng.sample(RandomBits::new(RANDOM_SECRET_LENGTH_BITS));

        // Convert password string to x number.
        // let x = BigUint::from_bytes_be(&Sha256::digest(password.as_bytes()));
        // let x = Bytes::from(Sha256::digest(password.as_bytes()).to_vec());
        // TODO: this is temporary...
        // let x = prover.generate_registration_x();
        // let x: Bytes = Bytes::from(password);
        let x = prover.compute_registration_x(password);

        Ok(Client {
            client,
            prover,
            user,
            x,
        })
    }

    pub async fn register(&self) -> Result<(), Status> {
        // let p = &self.parameters.p;
        // let g = &self.parameters.g;
        // let h = &self.parameters.h;

        // Compute y1 and y2 for registration.
        // let signed_x: BigInt = self.x.clone().into();
        // let y1 = g.modpow(&signed_x, p);
        // let y2 = h.modpow(&signed_x, p);
        let (y1, y2) = self.prover.compute_registration_y1y2(self.x.clone());
        log::debug!("y1 = {:?}", y1);
        log::debug!("y2 = {:?}", y2);

        // Send register request.
        let resp = self
            .client
            .clone()
            .register(RegisterRequest {
                user: self.user.clone(),
                y1,
                y2,
            })
            .await?
            .into_inner();

        log::debug!("{:?}", resp);

        Ok(())
    }

    pub async fn login(&self) -> Result<(), Status> {
        // Generate random number k.
        // Should not be negative because it's used as an exponent.
        // let mut rng = rand::thread_rng();
        // let k: BigUint = rng.sample(RandomBits::new(32));
        // let signed_k: BigInt = k.clone().into();
        let k = self.prover.generate_challenge_k();

        // let p = &self.parameters.p;
        // let q = &self.parameters.q;
        // let g = &self.parameters.g;
        // let h = &self.parameters.h;

        // Compute commitment (r1, r2) for authentication challenge.
        let (r1, r2) = self.prover.compute_challenge_commitment_r1r2(k.clone());
        // let r1 = g.modpow(&signed_k, p);
        // let r2 = h.modpow(&signed_k, p);
        log::debug!("r1 = {:?}", r1);
        log::debug!("r2 = {:?}", r2);

        // Send create_authentication_challenge request.
        let resp = self
            .client
            .clone()
            .create_authentication_challenge(AuthenticationChallengeRequest {
                user: self.user.clone(),
                r1,
                r2,
            })
            .await?
            .into_inner();

        log::debug!("{:?}", resp);

        // let c = resp.c.parse::<BigInt>().unwrap();
        let c = Bytes::from(resp.c);

        // Compute challenge response s.
        // Should not be negative because it's used as an exponent.
        let s = self
            .prover
            .compute_challenge_response_s(self.x.clone(), k, c);
        // let signed_x: BigInt = self.x.clone().into();
        // let mut s = (signed_k - c * signed_x) % q;
        // if s < Zero::zero() {
        //     s += q;
        // }
        log::debug!("s = {:?}", s);

        // Send verify_authentication request.
        let resp = self
            .client
            .clone()
            .verify_authentication(AuthenticationAnswerRequest {
                auth_id: resp.auth_id,
                s,
            })
            .await?
            .into_inner();

        log::debug!("{:?}", resp);

        Ok(())
    }
}

fn bytes_to_bigint(v: Bytes) -> BigInt {
    BigInt::from_bytes_be(Sign::Plus, &v)
}

fn bytes_to_ristretto_point(v: Bytes) -> RistrettoPoint {
    // if x.len() != 32 {}
    // TODO: make sure length is 32
    // TODO: fix these hard unwraps
    let v: [u8; 32] = v.as_ref().try_into().unwrap();
    let v = CompressedRistretto(v).decompress().unwrap();
    v
}

// #[cfg(test)]
// mod new {
//     use super::*;
//     use crate::test::mock_client;
//     use anyhow::Result;
//     use num_traits::One;

//     #[tokio::test]
//     async fn succeeds() -> Result<()> {
//         let client = mock_client().await?;
//         let prover = Client::new(client, "user".to_string(), "password".to_string())
//             .await
//             .unwrap();

//         assert_eq!(
//             prover.parameters,
//             Parameters {
//                 p: One::one(),
//                 q: One::one(),
//                 g: One::one(),
//                 h: One::one(),
//             }
//         );

//         Ok(())
//     }
// }

// #[cfg(test)]
// mod register {
//     use super::*;
//     use crate::test::mock_client;
//     use anyhow::Result;

//     #[tokio::test]
//     async fn succeeds() -> Result<()> {
//         let client = mock_client().await?;
//         let prover = Client::new(client, "user".to_string(), "password".to_string())
//             .await
//             .unwrap();

//         prover.register().await.unwrap();

//         Ok(())
//     }
// }

// #[cfg(test)]
// mod login {
//     use super::*;
//     use crate::test::mock_client;
//     use anyhow::Result;

//     #[tokio::test]
//     async fn succeeds() -> Result<()> {
//         let client = mock_client().await?;
//         let prover = Client::new(client, "user".to_string(), "password".to_string())
//             .await
//             .unwrap();

//         prover.login().await.unwrap();

//         Ok(())
//     }
// }
