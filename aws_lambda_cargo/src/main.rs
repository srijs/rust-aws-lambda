#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate rustc_version;
#[macro_use]
extern crate askama;
extern crate cargo_metadata;
extern crate tempfile;
extern crate users;
#[macro_use]
extern crate duct;

mod commands;
mod docker;
mod manifest_info;

use rustc_version::{Channel, version_meta};
use askama::Template;
use commands::build::Settings as BuildSettings;
use commands::check::{Scope, Settings as CheckSettings};
use docker::{DockerDynamicTemplate, DockerRunner};
use failure::Error;
use manifest_info::ManifestInfo;
use rustc_version::version;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use users::{get_current_uid, get_user_by_uid};
use std::env;


#[derive(Debug, Fail)]
enum ProgramError {
    #[fail(display = "the manifest-path must be a path to a Cargo.toml file")]
    ManifestPathNotFound,
    #[fail(display = "unable to get the current user and group")]
    GetCurrentUserFailed,
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
/// Easily use Rust with AWS Lambda
struct Cli {
    #[structopt(name = "PATH", long = "manifest-path", parse(from_os_str))]
    manifest_path: Option<PathBuf>,
    #[structopt(name = "CARGO COMMAND")]
    cargo_command: Vec<String>,
}

fn inner_main(args: Cli) -> Result<(), Error> {
    // Parse Cargo.toml for binary definitions.
    /*
    let metadata = cargo_metadata::metadata(args.manifest_path)
        .map_err(|_| ProgramError::ManifestPathNotFound)?;
    let manifest_info = ManifestInfo::new(metadata)?;
    debug!("Manifest info:\n{:#?}", manifest_info);
    */


    // Process the dockerfile template.
    let v = version_meta()?;
    let v = match v.channel {
        Channel::Beta => format!("beta-{}", v.commit_date.expect("rustc has commit date")),
        Channel::Dev => format!("dev-{}", v.commit_date.expect("rustc has commit date")),
        Channel::Nightly => format!("nightly-{}", v.commit_date.expect("rustc has commit date")),
        Channel::Stable => v.semver.to_string(),
    };

    let x = DockerDynamicTemplate { rust_version: &v };
    let dockerfile = x.render()?;
    debug!("Dockerfile:\n{}", dockerfile);

    let user = get_user_by_uid(get_current_uid()).ok_or(ProgramError::GetCurrentUserFailed)?;

    let mut runner = DockerRunner::new(dockerfile);
    runner.validate()?;
    let docker_image = runner.prepare_image()?;
    runner.run(
        &args.cargo_command.join(" "),
        &docker_image,
        &args.manifest_path.unwrap_or(env::current_dir().expect("cwd")),
        &user,
    )?;

    Ok(())
}

fn main() {
    drop(pretty_env_logger::init());
    let args = Cli::from_args();
    ::std::process::exit(match inner_main(args) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        }
    });
}
