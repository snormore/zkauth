use anyhow::Result;
use clap::ValueEnum;
use std::fs::File;
use strum_macros::{Display, EnumString, VariantNames};
use zkauth::{
    discrete_logarithm::configuration::DiscreteLogarithmConfiguration,
    elliptic_curve::configuration::EllipticCurveConfiguration,
};
use zkauth_protobuf::v1::Configuration;

/// Configuration flavor.
#[derive(Debug, Clone, EnumString, Display, VariantNames, ValueEnum, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum ConfigFlavor {
    DiscreteLogarithm,
    EllipticCurve,
}

/// Loads a configuration from a file.
pub fn load_config_from_file(path: &str) -> Result<Configuration> {
    let file = File::open(path)?;
    let config: Configuration = serde_json::from_reader(file)?;
    Ok(config)
}

/// Generates a configuration.
pub fn generate_config(flavor: ConfigFlavor, prime_bits: usize) -> Result<Configuration> {
    let config: Configuration = match flavor {
        ConfigFlavor::DiscreteLogarithm => {
            let config = DiscreteLogarithmConfiguration::generate(prime_bits);
            config.into()
        }
        ConfigFlavor::EllipticCurve => {
            let config = EllipticCurveConfiguration::generate();
            config.into()
        }
    };
    Ok(config)
}

/// Writes a configuration to a file.
pub fn write_config_to_file(config: Configuration, path: &str) -> Result<()> {
    serde_json::to_writer_pretty(File::create(path)?, &config)?;
    Ok(())
}

#[cfg(test)]
mod config {
    use super::*;
    use anyhow::Result;
    use tempdir::TempDir;
    use zkauth_protobuf::v1::configuration::Flavor;

    #[test]
    fn generate_discrete_logarithm_flavor() -> Result<()> {
        let config = generate_config(ConfigFlavor::DiscreteLogarithm, 8)?;
        assert!(config.flavor.is_some());
        assert!(matches!(config.flavor, Some(Flavor::DiscreteLogarithm(_))));

        Ok(())
    }

    #[test]
    fn generate_elliptic_curve_flavor() -> Result<()> {
        let config = generate_config(ConfigFlavor::EllipticCurve, 0)?;
        assert!(config.flavor.is_some());
        assert!(matches!(config.flavor, Some(Flavor::EllipticCurve(_))));

        Ok(())
    }

    #[test]
    fn write_to_and_load_from_file() -> Result<()> {
        let tmp_dir = TempDir::new("zkauth-test")?;
        let config_path = tmp_dir.path().join("config.json");
        let config_path = config_path.to_str().unwrap();

        let config = generate_config(ConfigFlavor::DiscreteLogarithm, 8)?;
        write_config_to_file(config.clone(), config_path)?;
        let loaded_config = load_config_from_file(config_path)?;

        assert_eq!(config, loaded_config);

        tmp_dir.close()?;
        Ok(())
    }
}
