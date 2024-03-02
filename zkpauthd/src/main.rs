use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Env;
use tokio::net::TcpListener;
use tonic::transport::Server;
use zkpauthd::Verifier;
use zkpauthpb::v1::auth_server::AuthServer;

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
        .add_service(AuthServer::new(Verifier::generated(256)))
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
        Ok(())
    }

    #[test]
    fn port_0() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-p=0"]);
        assert_eq!(opts.port, 0);
        Ok(())
    }

    #[test]
    fn port_25() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-p=25"]);
        assert_eq!(opts.port, 25);
        Ok(())
    }

    #[test]
    fn host_test_net() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--host=test.net"]);
        assert_eq!(opts.host, "test.net");
        Ok(())
    }

    #[test]
    fn host_0000() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--host=0.0.0.0"]);
        assert_eq!(opts.host, "0.0.0.0");
        Ok(())
    }
}
