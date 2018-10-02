use docker::DockerRunner;
use failure::Error;
use std::path::PathBuf;
use std::process::Output;

#[derive(Debug, StructOpt)]
pub struct Settings {
    #[structopt(name = "CARGO BUILD ARGS")]
    /// Arguments to pass through to `cargo build`
    cargo_args: Vec<String>,
}

fn dev_set(args: &Vec<String>) -> bool {
    args.iter().any(|x| x == "--dev")
}

fn target_set(args: &Vec<String>) -> bool {
    args.iter().any(|x| x == "--target")
}

pub fn run(
    settings: &Settings,
    runner: &mut DockerRunner,
    work_dir: &PathBuf,
    target_triple: &str,
) -> Result<Output, Error> {
    trace!("Chose `build` command");

    let mut command = vec!["cargo".to_string(), "build".to_string()];

    // Unless they choose a dev build, default to release.
    // This gets rid of a perf footgun.
    // TODO: Is this confusing?
    if !dev_set(&settings.cargo_args) {
        command.push("--release".to_string())
    }

    // Unless they choose a target, default to the correct one.
    // TODO: Is this confusing?
    if !target_set(&settings.cargo_args) {
        command.append(&mut vec!["--target".to_string(), target_triple.to_string()]);
    }

    // TODO: Use `out-dir` when stable <https://github.com/rust-lang/cargo/issues/6100>.

    command.append(&mut settings.cargo_args.clone());
    let output = runner.run(&command, &work_dir)?;
    Ok(output)
}
