use anyhow::{Error, Result};
use futures_util::FutureExt;
use std::path::Path;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::oneshot;
use tonic::transport::Server;
use zkauth::discrete_logarithm::verifier::DiscreteLogarithmVerifier;
use zkauth::elliptic_curve::verifier::EllipticCurveVerifier;
use zkauth::Verifier;
use zkauth_protobuf::v1::auth_server::AuthServer;
use zkauth_protobuf::v1::{configuration::Flavor, Configuration};

use crate::{
    cli::{
        config::{generate_config, load_config_from_file, write_config_to_file},
        options::Options,
    },
    service::Service,
};

/// Runs the server CLI with the specified options.
pub async fn run(opts: Options) -> Result<()> {
    opts.init_logger();

    // Check if a configuration file should be generated.
    let config_path = opts.config_path.clone().unwrap_or("".to_string());
    if opts.config_generate {
        if opts.config_path.is_none() {
            log::error!("Configuration file path is required when using --config-generate.");
            return Ok(());
        }

        if Path::new(config_path.as_str()).exists() && !opts.config_overwrite {
            log::error!(
                "Configuration file already exists at '{}'. Use --config-overwrite to overwrite.",
                config_path
            );
            return Ok(());
        } else {
            if opts.config_overwrite {
                log::warn!("Overwriting configuration file at '{}'.", config_path);
            }

            // Generate and write a new configuration file, and exit.
            let config = generate_config(opts.config_flavor, opts.config_prime_bits)?;
            write_config_to_file(config, config_path.as_str())?;
            log::info!("Configuration file generated at '{}'.", config_path);
            return Ok(());
        }
    }

    // Load configuration from file if specified, or generate a non-persistent configuration.
    let config: Configuration = if opts.config_path.is_some() {
        // Load configuration from file.
        if !Path::new(config_path.as_str()).exists() {
            log::error!("Configuration file not found at '{}'.", config_path);
            return Ok(());
        }

        load_config_from_file(config_path.as_str())?
    } else {
        // Generate a non-persistent configuration.
        log::info!("No configuration file specified, generating non-persistent configuration.");
        generate_config(opts.config_flavor, opts.config_prime_bits)?
    };
    let config_json = serde_json::to_string_pretty(&config).map_err(|e| {
        log::error!("Failed to serialize configuration: {}", e);
        e
    })?;
    println!("{}", config_json);

    // Create a channel to signal shutdown.
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();

    // Spawn a task to listen for termination signals.
    let signal_task = tokio::spawn(async move {
        // Wait for SIGINT or SIGTERM signal.
        signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl_c signal");
        shutdown_sender
            .send(())
            .expect("Failed to send shutdown signal");
    });

    // Server setup.
    let addr = format!("{}:{}", opts.host, opts.port);
    let listener = TcpListener::bind(addr).await?;
    log::info!("✅ Server listening on {}", listener.local_addr()?);

    // Initialize the service verifier.
    let verifier: Box<dyn Verifier> = match config.clone().flavor {
        Some(Flavor::DiscreteLogarithm(config)) => {
            Box::new(DiscreteLogarithmVerifier::new(config.try_into().map_err(
                |_| Error::msg("Failed to convert discrete logarithm configuration"),
            )?))
        }
        Some(Flavor::EllipticCurve(config)) => {
            Box::new(EllipticCurveVerifier::new(config.try_into().map_err(
                |_| Error::msg("Failed to convert elliptic curve configuration"),
            )?))
        }
        None => return Err(Error::msg("unknown configuration")),
    };

    // Initialize service and start the server.
    let service = Service::new(config, verifier);
    let server = Server::builder()
        .add_service(AuthServer::new(service))
        .serve_with_incoming_shutdown(
            tokio_stream::wrappers::TcpListenerStream::new(listener),
            shutdown_receiver.map(|_| ()),
        );

    // Run the server and wait for either completion or a shutdown signal.
    tokio::select! {
        _ = server => {
            log::info!("Server has shut down.");
        },
        _ = signal_task => {
            log::info!("Signal received, shutting down.");
        },
    }

    Ok(())
}
