#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate rustc_version;
#[macro_use]
extern crate askama;
extern crate cargo_metadata;
extern crate tempfile;

mod commands;
mod docker;
mod manifest_info;

use askama::Template;
use commands::build::Settings as BuildSettings;
use commands::check::{Scope, Settings as CheckSettings};
use docker::{DockerDynamicTemplate, DockerRunner};
use failure::Error;
use manifest_info::ManifestInfo;
use rustc_version::{version, Version};
use std::env;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Debug, Fail)]
enum ProgramError {
    #[fail(display = "the manifest-path must be a path to a Cargo.toml file")]
    ManifestPathNotFound,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "build")]
    /// Compile a local package and all of its dependencies for AWS Lambda.
    Build(BuildSettings),
    #[structopt(name = "check")]
    /// Check docker setup as well as a local package and all of its dependencies for errors.
    Check(CheckSettings),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-lambda")]
#[structopt(
    raw(global_settings = "&[AppSettings::VersionlessSubcommands, AppSettings::InferSubcommands]")
)]
/// Interact with AWS Lambda
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(name = "PATH", long = "manifest-path", parse(from_os_str))]
    manifest_path: Option<PathBuf>,
}

fn inner_main(args: Cli) -> Result<(), Error> {
    // Parse Cargo.toml for binary definitions.
    let metadata = cargo_metadata::metadata(args.manifest_path)
        .map_err(|_| ProgramError::ManifestPathNotFound)?;
    let manifest_info = ManifestInfo::new(metadata)?;

    // Process the dockerfile template.
    let v = version()?;
    let x = DockerDynamicTemplate { rust_version: &v };
    let dockerfile = x.render()?;
    debug!("{}", dockerfile);

    let mut runner = DockerRunner::new(dockerfile);

    match args.cmd {
        Command::Build(x) => {
            // We want to check to make sure docker is set up correctly.
            commands::check::run(
                &CheckSettings::default(),
                &manifest_info,
                &mut runner,
                Scope::Docker,
            )?;
            commands::build::run(&x)?;
        }
        Command::Check(x) => {
            commands::check::run(&x, &manifest_info, &mut runner, Scope::Docker)?;
            commands::check::run(&x, &manifest_info, &mut runner, Scope::Rust)?;
        }
    }
    Ok(())
}

fn main() {
    drop(env_logger::init());
    let args = Cli::from_args();
    inner_main(args).unwrap();
}
