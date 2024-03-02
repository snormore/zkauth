#[cfg(test)]
mod main {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn no_server_fails() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("zkauth-demo-cli")?;

        cmd.arg("--address")
            .arg("http://127.0.0.1:567812")
            .arg("--user")
            .arg("user")
            .arg("--password")
            .arg("password")
            .arg("--register");
        cmd.assert().failure().stderr(predicate::str::contains(
            "tcp connect error: Connection refused",
        ));

        Ok(())
    }
}
