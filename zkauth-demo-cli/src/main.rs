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

    /// Specifies the address of the gRPC server to connect to. Example: http://127.0.0.1:50001
    #[arg(short, long, env("ZKAUTH_ADDRESS"))]
    address: String,

    /// Specifies the username to authenticate with.
    #[arg(short, long, env("ZKAUTH_USER"))]
    user: String,

    /// Specifies the password to authenticate with.
    #[arg(short, long, env("ZKAUTH_PASSWORD"))]
    password: String,

    /// Specifies whether to execute the registration step.
    #[arg(long, default_value_t = false)]
    register: bool,

    /// Specifies whether to execute the login step.
    #[arg(long, default_value_t = false)]
    login: bool,
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

    if !opts.register && !opts.login {
        eprintln!("Error: Either --register or --login should be true");
        std::process::exit(1);
    }

    run(
        opts.address,
        opts.user,
        opts.password,
        opts.register,
        opts.login,
    )
    .await?;

    Ok(())
}

#[cfg(test)]
mod options {
    use super::*;

    #[test]
    fn host_https_test_net_5000() -> Result<()> {
        let opts = Options::parse_from(vec![
            "bin",
            "--address=https://test.net:5000",
            "--user=user",
            "--password=password",
        ]);
        assert_eq!(opts.address, "https://test.net:5000");
        Ok(())
    }

    #[test]
    fn init_logger_defaults() -> Result<()> {
        let opts = Options::parse_from(vec![
            "bin",
            "--address=https://test.net:5000",
            "--user=user",
            "--password=password",
        ]);
        opts.init_logger();
        Ok(())
    }

    #[test]
    fn init_logger_verbose() -> Result<()> {
        let opts = Options::parse_from(vec![
            "bin",
            "--address=https://test.net:5000",
            "--user=user",
            "--password=password",
            "-v",
        ]);
        opts.init_logger();
        Ok(())
    }
}
