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
    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,

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

    let addr = format!("{}:{}", opts.bind, opts.port);
    let listener = TcpListener::bind(addr).await?;

    log::info!("✅ Server listening on {}", listener.local_addr()?);

    Server::builder()
        .add_service(AuthServer::new(Verifier::new()))
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
        .await?;

    Ok(())
}
