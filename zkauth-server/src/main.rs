use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Env;
use futures_util::FutureExt;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::oneshot;
use tonic::transport::Server;
use zkauth::discrete_logarithm::{
    configuration::DiscreteLogarithmConfiguration, verifier::DiscreteLogarithmVerifier,
};
use zkauth::elliptic_curve::{
    configuration::EllipticCurveConfiguration, ristretto_point_to_bigint,
    verifier::EllipticCurveVerifier,
};
use zkauth_pb::v1::auth_server::AuthServer;
use zkauth_pb::v1::{configuration, Configuration};
use zkauth_server::service::Service;

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

    /// Specifies the number of bits to use for generating prime numbers for the public parameters.
    #[arg(long, default_value_t = 128)]
    prime_bits: usize,
}

impl Options {
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

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::parse();
    opts.init_logger();

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

    // TODO: set up configuration/verifier in a better way
    // let config = DiscreteLogarithmConfiguration::generate(opts.prime_bits);
    // // TODO: do this via into/from
    // let config_pb = Configuration {
    //     operations: Some(configuration::Operations::DiscreteLogarithm(
    //         configuration::DiscreteLogarithm {
    //             p: config.p.to_string(),
    //             q: config.q.to_string(),
    //             g: config.g.to_string(),
    //             h: config.h.to_string(),
    //         },
    //     )),
    // };
    // let service = Service::new(config_pb, Box::new(DiscreteLogarithmVerifier::new(config)));

    let config = EllipticCurveConfiguration::generate(opts.prime_bits);
    // TODO: do this via into/from
    let config_pb = Configuration {
        operations: Some(configuration::Operations::EllipticCurve(
            configuration::EllipticCurve {
                g: ristretto_point_to_bigint(config.g).to_string(),
                h: ristretto_point_to_bigint(config.h).to_string(),
            },
        )),
    };
    let service = Service::new(config_pb, Box::new(EllipticCurveVerifier::new(config)));

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
        assert_eq!(opts.prime_bits, 128);
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
    fn prime_bits_32() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--prime-bits=32"]);
        assert_eq!(opts.prime_bits, 32);
        Ok(())
    }
}
