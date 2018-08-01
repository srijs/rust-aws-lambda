use quicli::prelude::*;
use std;
use std::process::{self, Command, ExitStatus, Stdio};

#[derive(Debug, Fail)]
enum DockerError {
    #[fail(display = "docker is not installed or running: {}", error)]
    BinaryNotFound { error: std::io::Error },
    #[fail(display = "`docker --version` failed: {}", status)]
    VersionCommandStatus { status: ExitStatus },
}

pub enum Scope {
    All,
    Docker,
}

fn check_docker() -> Result<()> {
    let status = Command::new("docker")
        .args(vec!["--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| DockerError::BinaryNotFound { error: e })?;
    if (status.success()) {
        Ok(())
    } else {
        Err(DockerError::VersionCommandStatus { status })?
    }
}
#[derive(Debug, Default, StructOpt)]
pub struct Settings {
    #[structopt(name = "CARGO_OPTIONS")]
    /// Options to pass through to `cargo check`
    cargo_options: Vec<String>,
}

pub fn run(settings: &Settings, scope: Scope) -> Result<()> {
    println!("Running check");
    match scope {
        Scope::All => {
            check_docker()?;
            // TODO: Run `cargo check` in the docker container.
        }
        Scope::Docker => check_docker()?,
    }
    Ok(())
}
