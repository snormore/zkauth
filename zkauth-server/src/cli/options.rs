use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use env_logger::Env;

use super::config::ConfigFlavor;

/// Command line options for the server.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    /// Specifies the IP address or name of the host to which the server is bound.
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Specifies the TCP/IP port number on which the server listens for incoming client requests.
    #[arg(short, long, env("PORT"), default_value_t = 0)]
    pub port: u16,

    /// Specifies the configuration file path.
    /// If not specified, a non-persistent configuration will be generated and used.
    #[arg(long, env("CONFIG_PATH"))]
    pub config_path: Option<String>,

    /// Specifies whether to generate a new configuration file at the specified path.
    /// If true, this will exit after generating the configuration file, and not run the server.
    /// If the file already exists, it will not be overwritten unless the --config-overwrite is
    /// specified.
    #[arg(long, default_value_t = false)]
    pub config_generate: bool,

    /// Specifies whether to overwrite an existing configuration file when generating a new one.
    #[arg(long, default_value_t = false)]
    pub config_overwrite: bool,

    /// Specifies the configuration flavor to use.
    #[arg(long, default_value_t = ConfigFlavor::DiscreteLogarithm, value_enum)]
    pub config_flavor: ConfigFlavor,

    /// Specifies the number of bits to use for generating prime numbers for the public parameters.
    #[arg(long, default_value_t = 64)]
    pub config_prime_bits: usize,

    /// Specifies a prime number to use for generating the configuration.
    #[arg(long)]
    pub config_prime: Option<String>,
}

/// Implementation of the options.
impl Options {
    /// Initializes the logger based on the verbosity level.
    pub fn init_logger(&self) {
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

#[cfg(test)]
mod options {
    use super::*;
    use anyhow::Result;

    #[test]
    fn defaults() -> Result<()> {
        let opts = Options::parse_from(vec!["bin"]);
        assert_eq!(opts.port, 0);
        assert_eq!(opts.host, "127.0.0.1");
        assert_eq!(opts.config_prime_bits, 64);
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

    #[test]
    fn init_logger_verbose() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-v"]);
        opts.init_logger();
        Ok(())
    }

    #[test]
    fn init_logger_quiet() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-q"]);
        opts.init_logger();
        Ok(())
    }

    #[test]
    fn init_logger_debug() -> Result<()> {
        let opts = Options::parse_from(vec!["bin", "-vv"]);
        opts.init_logger();
        Ok(())
    }
}
