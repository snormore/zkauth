use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Env;
use tokio::net::TcpListener;
use tonic::transport::Server;
use zkauth_pb::v1::auth_server::AuthServer;
use zkauth_server::Verifier;

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

    let addr = format!("{}:{}", opts.host, opts.port);
    let listener = TcpListener::bind(addr).await?;

    log::info!("✅ Server listening on {}", listener.local_addr()?);

    Server::builder()
        .add_service(AuthServer::new(Verifier::generated(opts.prime_bits)))
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
        .await?;

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
