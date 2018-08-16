use docker::DockerRunner;
use failure::Error;
use manifest_info::ManifestInfo;
use users::{get_current_uid, get_user_by_uid};

#[derive(Debug, Fail)]
enum CommandError {
    #[fail(display = "unable to get the current user and group")]
    GetCurrentUserFailed,
}

pub enum Scope {
    Docker,
    Rust,
}

fn check_docker(runner: &mut DockerRunner) -> Result<(), Error> {
    info!("Checking docker install");
    runner.validate()
}

fn check_rust(runner: &mut DockerRunner, manifest_info: &ManifestInfo) -> Result<(), Error> {
    info!("Running `cargo check` in the docker container");
    // Make the docker image.
    let image_name = runner.prepare_image()?;

    let user = get_user_by_uid(get_current_uid()).ok_or(CommandError::GetCurrentUserFailed)?;
    
    let _binary = runner.run(
        "cargo check",
        &image_name,
        &manifest_info.source_location,
        &user,
    )?;
    Ok(())
}

#[derive(Debug, Default, StructOpt)]
pub struct Settings {
    #[structopt(name = "CARGO_OPTIONS")]
    /// Options to pass through to `cargo check`
    cargo_options: Vec<String>,
}

pub fn run(
    _settings: &Settings,
    manifest_info: &ManifestInfo,
    runner: &mut DockerRunner,
    scope: Scope,
) -> Result<(), Error> {
    trace!("Running `check` command");
    match scope {
        Scope::Docker => check_docker(runner),
        Scope::Rust => check_rust(runner, &manifest_info),
    }
}
