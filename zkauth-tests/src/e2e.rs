use anyhow::Result;
use tokio::net::TcpListener;
use tonic::transport::{Channel, Server};
use tonic::Code;
use zkauth_client::{prover::Prover, AuthClient};
use zkauth_pb::v1::auth_server::AuthServer;
use zkauth_server::Verifier;

async fn start_server_in_background() -> Result<AuthClient<Channel>> {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap().to_string());

    tokio::spawn(async move {
        Server::builder()
            .add_service(AuthServer::new(Verifier::default()))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await
    });

    let client = AuthClient::connect(address).await.unwrap();

    Ok(client)
}

#[tokio::test]
async fn register_login_succeeds() -> Result<()> {
    let client = start_server_in_background().await.unwrap();

    let prover = Prover::new(client, "user".to_string(), "password".to_string())
        .await
        .unwrap();

    prover.register().await.unwrap();
    prover.login().await.unwrap();

    Ok(())
}

#[tokio::test]
async fn new_fails_with_empty_user() -> Result<()> {
    let client = start_server_in_background().await.unwrap();

    let err = Prover::new(client, "".to_string(), "password".to_string())
        .await
        .unwrap_err();
    assert_eq!(err.code(), Code::InvalidArgument);
    assert_eq!(err.message(), "Invalid user argument");

    Ok(())
}

#[tokio::test]
async fn new_fails_with_empty_password() -> Result<()> {
    let client = start_server_in_background().await.unwrap();

    let err = Prover::new(client, "user".to_string(), "".to_string())
        .await
        .unwrap_err();
    assert_eq!(err.code(), Code::InvalidArgument);
    assert_eq!(err.message(), "Invalid password argument");

    Ok(())
}

#[tokio::test]
async fn login_fails_when_not_registered() -> Result<()> {
    let client = start_server_in_background().await.unwrap();

    let prover = Prover::new(client, "user".to_string(), "password".to_string())
        .await
        .unwrap();

    let err = prover.login().await.unwrap_err();
    assert_eq!(err.code(), Code::NotFound);
    assert_eq!(err.message(), "User not found");

    Ok(())
}

#[tokio::test]
async fn register_twice_with_same_user_fails() -> Result<()> {
    let client = start_server_in_background().await.unwrap();

    let prover = Prover::new(client, "user".to_string(), "password".to_string())
        .await
        .unwrap();

    prover.register().await.unwrap();

    let err = prover.register().await.unwrap_err();
    assert_eq!(err.code(), Code::AlreadyExists);
    assert_eq!(err.message(), "User already registered");

    Ok(())
}

#[tokio::test]
async fn register_login_login_succeeds() -> Result<()> {
    let client = start_server_in_background().await.unwrap();

    let prover = Prover::new(client, "user".to_string(), "password".to_string())
        .await
        .unwrap();

    prover.register().await.unwrap();
    prover.login().await.unwrap();
    prover.login().await.unwrap();

    Ok(())
}

#[tokio::test]
async fn register_login_twice_with_different_users_succeeds() -> Result<()> {
    let client = start_server_in_background().await.unwrap();

    let prover1 = Prover::new(client.clone(), "user1".to_string(), "password".to_string())
        .await
        .unwrap();

    let prover2 = Prover::new(client, "user2".to_string(), "password".to_string())
        .await
        .unwrap();

    prover1.register().await.unwrap();
    prover2.register().await.unwrap();

    prover1.login().await.unwrap();
    prover2.login().await.unwrap();

    Ok(())
}
