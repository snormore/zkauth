#[cfg(test)]
mod main {
    use assert_cmd::prelude::*;
    use std::io::Read;
    use std::{
        process::{Command, Stdio},
        time::Duration,
    };
    use wait_timeout::ChildExt;

    #[test]
    fn succeeds() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("zkauth-server")?
            .arg("--prime-bits=16")
            .stderr(Stdio::piped())
            .spawn()?;

        let _status_code = match cmd.wait_timeout(Duration::from_millis(500)).unwrap() {
            Some(status) => status.code(),
            None => {
                cmd.kill().unwrap();
                cmd.wait().unwrap().code()
            }
        };

        let mut output = String::new();
        cmd.stderr.unwrap().read_to_string(&mut output).unwrap();

        assert!(
            output.contains("Server listening on 127.0.0.1:"),
            "{}",
            output
        );

        Ok(())
    }
}
