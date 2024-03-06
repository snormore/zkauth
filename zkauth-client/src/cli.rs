//! A simple command-line interface for the zkauth client.
//!
//! This crate provides a simple command-line interface for the zkauth client. It allows users to
//! register and login using the zkauth protocol.
//!
//! # Usage
//!
//! ```sh
//! zkauth-client --address http://localhost:5001 --user user --password password --register --login
//! ```
//!
//! This command will register and login the user `user` with the password `password` using the
//! zkauth protocol at the address `http://localhost:5001`.

use crate::{client::Client, AuthClient};
use anyhow::Result;

/// Runs the zkauth client.
pub async fn run(
    address: String,
    user: String,
    password: String,
    register: bool,
    login: bool,
) -> Result<()> {
    let client = AuthClient::connect(address).await?;
    let prover = Client::new(client, user, password).await?;

    if register {
        prover.register().await?;
    }

    if login {
        prover.login().await?;
    }

    Ok(())
}

#[cfg(test)]
mod run {
    use super::*;
    use tokio::net::TcpListener;
    use tonic::transport::Server;
    use zkauth::discrete_logarithm::{
        configuration::DiscreteLogarithmConfiguration, verifier::DiscreteLogarithmVerifier,
    };
    use zkauth_protobuf::v1::auth_server::AuthServer;
    use zkauth_server::service::Service;

    fn test_service() -> Service {
        let config = DiscreteLogarithmConfiguration::generate(16);
        let verifier = Box::new(DiscreteLogarithmVerifier::new(config.clone()));
        Service::new(config.into(), verifier)
    }

    async fn start_server_in_background() -> Result<String> {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap().to_string());

        tokio::spawn(async move {
            Server::builder()
                .add_service(AuthServer::new(test_service()))
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                .await
        });

        Ok(address)
    }

    #[tokio::test]
    async fn succeeds() -> Result<()> {
        let address = start_server_in_background().await?;
        run(
            address,
            "user".to_string(),
            "password".to_string(),
            true,
            true,
        )
        .await?;
        Ok(())
    }
}
