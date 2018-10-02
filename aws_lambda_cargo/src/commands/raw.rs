use docker::DockerRunner;
use failure::Error;
use std::path::PathBuf;
use std::process::Output;

#[derive(Debug, Default, StructOpt)]
pub struct Settings {
    #[structopt(name = "SHELL COMMAND", raw(required = "true"))]
    /// The shell command to run in the container.
    shell_command_args: Vec<String>,
}

pub fn run<'a>(
    settings: &Settings,
    runner: &'a mut DockerRunner<'a>,
    work_dir: &PathBuf,
) -> Result<Output, Error> {
    trace!("Chose `raw` command");
    let output = runner.run(&settings.shell_command_args, work_dir)?;
    Ok(output)
}
