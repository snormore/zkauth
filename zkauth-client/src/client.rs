use num_bigint::BigInt;
use tonic::{transport::Channel, Status};
use zkauth::{
    discrete_logarithm::{
        configuration::DiscreteLogarithmConfiguration, prover::DiscreteLogarithmProver,
    },
    elliptic_curve::{
        bigint_to_ristretto_point, configuration::EllipticCurveConfiguration,
        prover::EllipticCurveProver,
    },
    Prover,
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
    x: BigInt,
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
                // TODO: do this via into/from
                DiscreteLogarithmConfiguration {
                    p: config.p.parse::<BigInt>().unwrap(),
                    q: config.q.parse::<BigInt>().unwrap(),
                    g: config.g.parse::<BigInt>().unwrap(),
                    h: config.h.parse::<BigInt>().unwrap(),
                },
            )),
            Some(Operations::EllipticCurve(config)) => {
                Box::new(EllipticCurveProver::new(EllipticCurveConfiguration {
                    // TODO: do this via into/from
                    g: bigint_to_ristretto_point(config.g.parse::<BigInt>().unwrap()),
                    h: bigint_to_ristretto_point(config.h.parse::<BigInt>().unwrap()),
                }))
            }
            None => return Err(Status::internal("unknown server configuration")),
        };

        // Convert password string to x number.
        let x = prover.compute_registration_x(password);

        Ok(Client {
            client,
            prover,
            user,
            x,
        })
    }

    pub async fn register(&self) -> Result<(), Status> {
        // Compute y1 and y2 for registration.
        let (y1, y2) = self.prover.compute_registration_y1y2(self.x.clone());
        log::debug!("y1 = {:?}", y1);
        log::debug!("y2 = {:?}", y2);

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
        let k = self.prover.generate_challenge_k();

        // Compute commitment (r1, r2) for authentication challenge.
        let (r1, r2) = self.prover.compute_challenge_commitment_r1r2(k.clone());
        log::debug!("r1 = {:?}", r1);
        log::debug!("r2 = {:?}", r2);

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

        log::debug!("{:?}", resp);

        let c = resp.c.parse::<BigInt>().unwrap();

        // Compute challenge response s.
        let s = self
            .prover
            .compute_challenge_response_s(self.x.clone(), k, c);
        log::debug!("s = {:?}", s);

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
