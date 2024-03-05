// Enforce documentation for all public items in the crate.
#![warn(missing_docs)]

//! Server for the zkauth authentication protocol.
//!
//! The server listens for incoming client requests and processes them using the zkauth protocol.
//! It uses the `tonic` library to provide a gRPC interface for the client to communicate with the
//! server.
//!
//! The server can be configured using command line options, and can also generate a configuration
//! file for use with the server. The configuration file specifies the public parameters for the
//! zkauth protocol, such as the prime number and generator for the discrete logarithm protocol, or
//! the base points for the elliptic curve protocol.
//!
//! # Usage
//!
//! The server can be run with the following command:
//!
//! ```sh
//! zkauth-server
//! ```
//!
//! The server can be configured using the following command line options:
//!
//! ```sh
//! zkauth-server --help
//! ```
//!
//! The server can also generate a configuration file using the following command:
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json
//! ```
//!
//! You can specify the configuration flavor using the `--config-flavor` option, and the number of
//! bits for the prime number using the `--config-prime-bits` option, or specify a prime number
//! directly using the `--config-prime` option.
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json --config-flavor=elliptic-curve
//! ```
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json --config-prime-bits=256
//! ```
//!
//! ```sh
//! zkauth-server --config-generate --config-path=config.json --config-prime=42765216643065397982265462252423826320512529931694366715111734768493812630447
//! ```

use anyhow::{Error, Result};
use clap::{Parser, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Env;
use futures_util::FutureExt;
use num_bigint::BigInt;
use std::fs::File;
use std::path::Path;
use strum_macros::{Display, EnumString, VariantNames};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::oneshot;
use tonic::transport::Server;
use zkauth::discrete_logarithm::{
    configuration::DiscreteLogarithmConfiguration, verifier::DiscreteLogarithmVerifier,
};
use zkauth::elliptic_curve::{
    configuration::EllipticCurveConfiguration, verifier::EllipticCurveVerifier,
};
use zkauth::Verifier;
use zkauth_protobuf::v1::auth_server::AuthServer;
use zkauth_protobuf::v1::{configuration::Flavor, Configuration};
use zkauth_server::service::Service;

/// Command line options for the server.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Specifies the IP address or name of the host to which the server is bound.
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Specifies the TCP/IP port number on which the server listens for incoming client requests.
    #[arg(short, long, env("PORT"), default_value_t = 0)]
    port: u16,

    /// Specifies the configuration file path.
    /// If not specified, a non-persistent configuration will be generated and used.
    #[arg(long, env("CONFIG_PATH"))]
    config_path: Option<String>,

    /// Specifies whether to generate a new configuration file at the specified path.
    /// If true, this will exit after generating the configuration file, and not run the server.
    /// If the file already exists, it will not be overwritten unless the --config-overwrite is
    /// specified.
    #[arg(long, default_value_t = false)]
    config_generate: bool,

    /// Specifies whether to overwrite an existing configuration file when generating a new one.
    #[arg(long, default_value_t = false)]
    config_overwrite: bool,

    /// Specifies the configuration flavor to use.
    #[arg(long, default_value_t = ConfigFlavor::DiscreteLogarithm, value_enum)]
    config_flavor: ConfigFlavor,

    /// Specifies the number of bits to use for generating prime numbers for the public parameters.
    #[arg(long, default_value_t = 256)]
    config_prime_bits: usize,

    /// Specifies a prime number to use for generating the configuration.
    #[arg(long)]
    config_prime: Option<String>,
}

/// Configuration flavor.
#[derive(Debug, Clone, EnumString, Display, VariantNames, ValueEnum, PartialEq)]
#[strum(serialize_all = "kebab-case")]
enum ConfigFlavor {
    DiscreteLogarithm,
    EllipticCurve,
}

/// Implementation of the options.
impl Options {
    /// Initializes the logger based on the verbosity level.
    fn init_logger(&self) {
        if self.verbose.is_present() {
            env_logger::Builder::new()
                .filter_level(self.verbose.log_level_filter())
                .init();
        } else {
            env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
        }
    }
}

/// Main entry point for the server.
#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::parse();
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

            // Generate a new configuration file and exit.
            let config: Configuration = match opts.config_flavor {
                ConfigFlavor::DiscreteLogarithm => {
                    let config = DiscreteLogarithmConfiguration::generate(opts.config_prime_bits);
                    config.into()
                }
                ConfigFlavor::EllipticCurve => {
                    let config = EllipticCurveConfiguration::generate();
                    config.into()
                }
            };
            serde_json::to_writer_pretty(File::create(config_path.as_str())?, &config)?;
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

        let file = File::open(config_path.clone())?;
        let config: Configuration = serde_json::from_reader(file)?;
        log::info!("Configuration loaded from '{}'.", config_path);
        let config_json = serde_json::to_string_pretty(&config).map_err(|e| {
            log::error!("Failed to serialize configuration: {}", e);
            e
        })?;
        println!("{}", config_json);

        serde_json::from_str(&config_json)?
    } else {
        // Generate a non-persistent configuration.
        log::info!("No configuration file specified, generating non-persistent configuration.");
        match opts.config_flavor {
            ConfigFlavor::EllipticCurve => {
                let config = EllipticCurveConfiguration::generate();
                let config = config.into();
                let config_json = serde_json::to_string_pretty(&config).map_err(|e| {
                    log::error!("Failed to serialize configuration: {}", e);
                    e
                })?;
                println!("{}", config_json);
                config
            }
            _ => {
                let config = match opts.config_prime {
                    Some(prime) => {
                        let prime: BigInt = prime.parse().expect("Failed to parse prime number");
                        log::info!("Using specified prime number for configuration: {}", prime);
                        DiscreteLogarithmConfiguration::generate_from_prime(prime)
                    }
                    None => DiscreteLogarithmConfiguration::generate(opts.config_prime_bits),
                };
                let config = config.into();
                let config_json = serde_json::to_string_pretty(&config).map_err(|e| {
                    log::error!("Failed to serialize configuration: {}", e);
                    e
                })?;
                println!("{}", config_json);
                config
            }
        }
    };

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

#[cfg(test)]
mod options {
    use super::*;

    #[test]
    fn defaults() -> Result<()> {
        let opts = Options::parse_from(vec!["bin"]);
        assert_eq!(opts.port, 0);
        assert_eq!(opts.host, "127.0.0.1");
        assert_eq!(opts.config_prime_bits, 256);
        Ok(())
    }

    #[test]
    fn port_0() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-p=0"]);
        assert_eq!(opts.port, 0);
        Ok(())
    }

    #[test]
    fn port_3000() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-p=3000"]);
        assert_eq!(opts.port, 3000);
        Ok(())
    }

    #[test]
    fn host_test_local() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--host=test.local"]);
        assert_eq!(opts.host, "test.local");
        Ok(())
    }

    #[test]
    fn host_0000() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--host=0.0.0.0"]);
        assert_eq!(opts.host, "0.0.0.0");
        Ok(())
    }

    #[test]
    fn config_prime_bits_32() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--config-prime-bits=32"]);
        assert_eq!(opts.config_prime_bits, 32);
        Ok(())
    }

    #[test]
    fn config_path() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--config-path=config.json"]);
        assert_eq!(opts.config_path, Some("config.json".to_string()));
        Ok(())
    }

    #[test]
    fn config_generate() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--config-generate"]);
        assert_eq!(opts.config_generate, true);
        Ok(())
    }

    #[test]
    fn config_overwrite() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--config-overwrite"]);
        assert_eq!(opts.config_overwrite, true);
        Ok(())
    }

    #[test]
    fn config_flavor_discrete_logarithm() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--config-flavor=discrete-logarithm"]);
        assert_eq!(opts.config_flavor, ConfigFlavor::DiscreteLogarithm);
        Ok(())
    }

    #[test]
    fn config_flavor_elliptic_curve() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--config-flavor=elliptic-curve"]);
        assert_eq!(opts.config_flavor, ConfigFlavor::EllipticCurve);
        Ok(())
    }

    #[test]
    fn config_flavor_default() -> Result<()> {
        let opts = Options::parse_from(vec!["bin"]);
        assert_eq!(opts.config_flavor, ConfigFlavor::DiscreteLogarithm);
        Ok(())
    }

    #[test]
    fn verbose() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-v"]);
        assert_eq!(opts.verbose.log_level_filter(), log::LevelFilter::Debug);
        Ok(())
    }
}
