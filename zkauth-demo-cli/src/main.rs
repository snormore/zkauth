use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Env;
use zkauth_demo_cli::run;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Specifies the address of the gRPC server to connect to.
    #[arg(short, long, default_value = "http://127.0.0.1:50001")]
    address: String,
}

impl Options {
    fn init_logger(&self) {
        if self.verbose.is_present() {
            let _ = env_logger::Builder::new()
                .filter_level(self.verbose.log_level_filter())
                .try_init();
        } else {
            let _ =
                env_logger::Builder::from_env(Env::default().default_filter_or("info")).try_init();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::parse();
    opts.init_logger();

    run(opts.address).await?;

    Ok(())
}

#[cfg(test)]
mod options {
    use super::*;

    #[test]
    fn defaults() -> Result<()> {
        let opts = Options::parse_from(vec!["bin"]);
        assert_eq!(opts.address, "http://127.0.0.1:50001");
        Ok(())
    }

    #[test]
    fn host_https_test_net_5000() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "--address=https://test.net:5000"]);
        assert_eq!(opts.address, "https://test.net:5000");
        Ok(())
    }

    #[test]
    fn init_logger_defaults() -> Result<()> {
        let opts = Options::parse_from(vec!["bin"]);
        opts.init_logger();
        Ok(())
    }

    #[test]
    fn init_logger_verbose() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-v"]);
        opts.init_logger();
        Ok(())
    }
}
