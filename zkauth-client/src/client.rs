use tonic::{transport::Channel, Status};
use zkauth::{
    discrete_logarithm::prover::DiscreteLogarithmProver,
    elliptic_curve::prover::EllipticCurveProver, Prover, Scalar,
};
use zkauth_protobuf::v1::{
    auth_client::AuthClient, configuration::Flavor, AuthenticationAnswerRequest,
    AuthenticationChallengeRequest, GetConfigurationRequest, RegisterRequest,
};

/// Client for the authentication protocol.
#[derive(Debug)]
pub struct Client {
    client: AuthClient<Channel>,
    prover: Box<dyn Prover>,
    user: String,
    x: Scalar,
}

/// Implementation of the client.
impl Client {
    /// Creates a new client given a user and password.
    /// # Errors
    /// * Returns an error if the user or password is invalid.
    /// * Returns an error if the configuration is unknown or cannot be converted.
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

        // Get the configuration from the server.
        let config = client
            .get_configuration(GetConfigurationRequest {})
            .await?
            .into_inner();
        let prover: Box<dyn Prover> = match config.flavor {
            Some(Flavor::DiscreteLogarithm(config)) => {
                Box::new(DiscreteLogarithmProver::new(config.try_into().map_err(
                    |_| Status::internal("failed to convert discrete logarithm configuration"),
                )?))
            }
            Some(Flavor::EllipticCurve(config)) => {
                Box::new(EllipticCurveProver::new(config.try_into().map_err(
                    |_| Status::internal("failed to convert elliptic curve configuration"),
                )?))
            }
            None => return Err(Status::internal("unknown configuration")),
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

    /// Registers the user by computing y1 and y2 and sending a registration request to the server.
    /// # Errors
    /// * Returns an error if the registration fails.
    /// * Returns an error if the y1 and y2 cannot be computed.
    /// * Returns an error if the registration request fails to the server.
    /// * Returns an error if the challenge response from the server is invalid.
    pub async fn register(&self) -> Result<(), Status> {
        // Compute y1 and y2 for registration.
        let (y1, y2) = self
            .prover
            .compute_registration_y1y2(self.x.clone())
            .map_err(|_| Status::internal("failed to compute registration y1 and y2"))?;
        log::info!("y1 = {:?}", y1);
        log::info!("y2 = {:?}", y2);

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

        log::info!("{:?}", resp);

        Ok(())
    }

    /// Logs in the user by sending a challenge request to the server and verifying the response.
    /// # Errors
    /// * Returns an error if the challenge response fails.
    /// * Returns an error if the verification fails.
    pub async fn login(&self) -> Result<(), Status> {
        // Generate random number k.
        let k = self.prover.generate_challenge_k();

        // Compute commitment (r1, r2) for authentication challenge.
        let (r1, r2) = self
            .prover
            .compute_challenge_commitment_r1r2(k.clone())
            .map_err(|_| Status::internal("failed to compute challenge commitment"))?;
        log::info!("r1 = {:?}", r1);
        log::info!("r2 = {:?}", r2);

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

        log::info!("{:?}", resp);

        let c: Scalar = resp
            .c
            .parse()
            .map_err(|_| Status::internal("failed to parse c"))?;

        // Compute challenge response s.
        let s = self
            .prover
            .compute_challenge_response_s(self.x.clone(), k, c)
            .map_err(|_| Status::internal("failed to compute challenge response s"))?;
        log::info!("s = {:?}", s);

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

        log::info!("{:?}", resp);

        Ok(())
    }
}

#[cfg(test)]
mod new {
    use super::*;
    use crate::test::mock_client;
    use anyhow::Result;

    /// Tests the new client creation.
    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let client = mock_client().await?;
        let prover = Client::new(client, "user".to_string(), "password".to_string())
            .await
            .unwrap();

        assert_eq!(prover.user, "user");

        Ok(())
    }
}

#[cfg(test)]
mod register {
    use super::*;
    use crate::test::mock_client;
    use anyhow::Result;

    /// Tests that the registration process succeeds.
    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let client = mock_client().await?;
        let prover = Client::new(client, "user".to_string(), "password".to_string())
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

    /// Tests that the login process succeeds.
    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let client = mock_client().await?;
        let prover = Client::new(client, "user".to_string(), "password".to_string())
            .await
            .unwrap();

        prover.login().await.unwrap();

        Ok(())
    }
}
