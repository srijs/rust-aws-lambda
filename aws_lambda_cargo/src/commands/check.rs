use askama::Template;
use cargo_metadata::Metadata;
use docker::{DockerDynamicTemplate, DockerRunner};
use failure::Error;
use manifest_info::ManifestInfo;
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
    Docker,
    Rust,
}

fn check_docker(runner: &mut DockerRunner) -> Result<(), Error> {
    info!("Checking docker install");
    runner.validate()
}

fn check_rust(runner: &mut DockerRunner) -> Result<(), Error> {
    info!("Running `cargo check` in the docker container");
    // Make the docker image.
    let image_name = runner.make_image()?;
    Ok(())
}

#[derive(Debug, Default, StructOpt)]
pub struct Settings {
    #[structopt(name = "CARGO_OPTIONS")]
    /// Options to pass through to `cargo check`
    cargo_options: Vec<String>,
}

pub fn run(
    settings: &Settings,
    manifest_info: &ManifestInfo,
    runner: &mut DockerRunner,
    scope: Scope,
) -> Result<(), Error> {
    trace!("Running `check` command");
    match scope {
        Scope::Docker => check_docker(runner),
        Scope::Rust => check_rust(runner),
    }
}
