#[macro_use]
extern crate structopt;
#[macro_use]
extern crate quicli;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

use quicli::prelude::*;
use structopt::clap::AppSettings;

mod commands;
use commands::build::Settings as BuildSettings;
use commands::check::{Scope, Settings as CheckSettings};

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
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    match args.cmd {
        Command::Build(x) => {
            // We want to check to make sure docker is set up correctly.
            commands::check::run(&CheckSettings::default(), Scope::Docker)?;
            commands::build::run(&x)?;
        }
        Command::Check(x) => commands::check::run(&x, Scope::All)?,
    }
});
