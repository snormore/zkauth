#[cfg(test)]
mod run {
    use anyhow::Result;
    use clap::Parser;
    use std::time::Duration;
    use tempdir::TempDir;
    use tokio::time::sleep;
    use zkauth_server::cli::{
        config::{generate_config, load_config_from_file, write_config_to_file, ConfigFlavor},
        run, Options,
    };

    #[tokio::test]
    async fn succeeds_with_generate_config() -> Result<()> {
        let tmp_dir = TempDir::new("zkauth-test")?;
        let config_path = tmp_dir.path().join("config.json");
        let config_path = config_path.to_str().unwrap();

        let opts = Options::parse_from(vec![
            "bin",
            "--config-prime-bits=8",
            "--config-generate",
            format!("--config-path={}", config_path).as_str(),
        ]);
        run(opts).await?;

        tmp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn generate_does_not_overwrite_existing_by_default() -> Result<()> {
        let tmp_dir = TempDir::new("zkauth-test")?;
        let config_path = tmp_dir.path().join("config.json");
        let config_path = config_path.to_str().unwrap();

        let config = generate_config(ConfigFlavor::DiscreteLogarithm, 8)?;
        write_config_to_file(config.clone(), config_path)?;

        let opts = Options::parse_from(vec![
            "bin",
            "--config-prime-bits=8",
            "--config-generate",
            format!("--config-path={}", config_path).as_str(),
        ]);
        run(opts).await?;

        let loaded_config = load_config_from_file(config_path)?;
        assert_eq!(config, loaded_config);

        tmp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn generate_overwrites_existing_with_option() -> Result<()> {
        let tmp_dir = TempDir::new("zkauth-test")?;
        let config_path = tmp_dir.path().join("config.json");
        let config_path = config_path.to_str().unwrap();

        let config = generate_config(ConfigFlavor::DiscreteLogarithm, 8)?;
        write_config_to_file(config.clone(), config_path)?;

        let opts = Options::parse_from(vec![
            "bin",
            "--config-prime-bits=8",
            "--config-generate",
            "--config-overwrite",
            format!("--config-path={}", config_path).as_str(),
        ]);
        run(opts).await?;

        let loaded_config = load_config_from_file(config_path)?;
        assert_ne!(config, loaded_config);

        tmp_dir.close()?;
        Ok(())
    }

    #[tokio::test]
    async fn succeeds_with_default_opts() -> Result<()> {
        tokio::spawn(async move {
            let opts = Options::parse_from(vec!["bin", "--config-prime-bits=8"]);
            run(opts).await
        });

        sleep(Duration::from_millis(200)).await;

        Ok(())
    }

    #[tokio::test]
    async fn succeeds_with_loaded_config() -> Result<()> {
        let tmp_dir = TempDir::new("zkauth-test")?;
        let config_path = tmp_dir.path().join("config.json");
        let config_path_owned = config_path.to_str().unwrap().to_owned();

        let config = generate_config(ConfigFlavor::DiscreteLogarithm, 8)?;
        write_config_to_file(config.clone(), &config_path_owned)?;

        tokio::spawn(async move {
            let opts = Options::parse_from(vec![
                "bin",
                "--config-prime-bits=8",
                format!("--config-path={}", config_path_owned).as_str(),
            ]);
            run(opts).await
        });

        sleep(Duration::from_millis(500)).await;

        let loaded_config = load_config_from_file(config_path.to_str().unwrap())?;
        assert_eq!(config, loaded_config);

        tmp_dir.close()?;
        Ok(())
    }
}
