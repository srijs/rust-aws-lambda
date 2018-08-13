use docker::DockerRunner;
use failure::Error;
use manifest_info::ManifestInfo;
use progress::Progress;

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
    let _image_name = runner.make_image()?;
    Ok(())
}

#[derive(Debug, Default, StructOpt)]
pub struct Settings {
    #[structopt(name = "CARGO_OPTIONS")]
    /// Options to pass through to `cargo check`
    cargo_options: Vec<String>,
}

pub fn run(
    _progress: &mut Progress,
    _settings: &Settings,
    _manifest_info: &ManifestInfo,
    runner: &mut DockerRunner,
    scope: Scope,
) -> Result<(), Error> {
    trace!("Running `check` command");
    match scope {
        Scope::Docker => check_docker(runner),
        Scope::Rust => check_rust(runner),
    }
}
