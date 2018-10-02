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
extern crate duct;
extern crate tempfile;
extern crate users;

mod commands;
mod docker;
mod templates;

use askama::Template;
use commands::build::Settings as BuildSettings;
use commands::raw::Settings as RawSettings;
use docker::DockerRunner;
use failure::Error;
use rustc_version::{version_meta, Channel};
use std::env;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use templates::{DockerDynamicTemplate, DockerStaticTemplate};
use templates::{MuslVersion, OpenSslVersion, RustupVersion};
use users::{get_current_uid, get_user_by_uid};

#[derive(Debug, Fail)]
enum ProgramError {
    #[fail(display = "unable to get the current user and group")]
    GetCurrentUserFailed,
    #[fail(display = "the manifest-path must be a path to a Cargo.toml file")]
    ManifestPathNotFound,
    #[fail(display = "directory does not exist: {}", loc)]
    WorkDirectoryDoesNotExist { loc: String },
    #[fail(display = "not a directory: {}", loc)]
    WorkDirectoryIsNotDirectory { loc: String },
}

arg_enum! {
    #[derive(Debug)]
    enum Link {
        Dynamic,
        Static,
    }
}

impl Link {
    pub fn target_triple(&self) -> String {
        match self {
            Link::Static => "x86_64-unknown-linux-musl".to_string(),
            Link::Dynamic => "x86_64-unknown-linux-gnu".to_string(),
        }
    }
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "build")]
    /// Compile a local package and all of its dependencies for AWS Lambda.
    Build(BuildSettings),
    #[structopt(name = "raw")]
    /// Run a raw shell command.
    Raw(RawSettings),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-lambda")]
#[structopt(
    raw(global_settings = "&[AppSettings::VersionlessSubcommands, AppSettings::InferSubcommands]")
)]
/// Easily use Rust with AWS Lambda
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
    /// Link style to use.
    /// Statically linked binaries are larger but more compatible.
    #[structopt(
        name = "STYLE",
        short = "link",
        long = "link-type",
        default_value = "Dynamic",
        raw(
            possible_values = "&Link::variants()",
            case_insensitive = "true"
        )
    )]
    link_type: Link,
    #[structopt(name = "PATH", long = "work-dir", parse(from_os_str))]
    work_dir: Option<PathBuf>,
    #[structopt(long = "rustup-version", name = "RUSTUP VERSION")]
    rustup_version: Option<RustupVersion>,
    #[structopt(
        long = "musl-version",
        default_value = "1.1.19",
        name = "MUSL VERSION"
    )]
    musl_version: MuslVersion,
    #[structopt(
        long = "openssl-version",
        default_value = "1.1.0i",
        name = "OPENSSL VERSION"
    )]
    openssl_version: OpenSslVersion,
    #[structopt(long = "image-name", name = "DOCKER IMAGE NAME")]
    image_name: Option<String>,
}

fn version_from_rustc() -> Result<RustupVersion, Error> {
    let v = version_meta()?;
    Ok(RustupVersion::new(&match v.channel {
        Channel::Beta => format!("beta-{}", v.commit_date.expect("rustc has commit date")),
        Channel::Dev => format!("dev-{}", v.commit_date.expect("rustc has commit date")),
        Channel::Nightly => format!("nightly-{}", v.commit_date.expect("rustc has commit date")),
        Channel::Stable => v.semver.to_string(),
    }))
}

/// TODO: This only checks direct deps.
/*
fn openssl_in_dependencies(p: Option<PathBuf>) -> Result<bool, Error> {
    let metadata = cargo_metadata::metadata(p.map(|x| x.join("Cargo.toml"))).map_err(|_| ProgramError::ManifestPathNotFound)?;
    debug!("Metadata:\n{:#?}", metadata);
    Ok(metadata
        .packages
        .iter()
        .flat_map(|x| x.dependencies.clone())
        .any(|x| x.name == "openssl" || x.name == "openssl-sys" || x.name.contains("openssl")))
}
*/

fn inner_main(args: Cli) -> Result<(), Error> {
    debug!("Arguments:\n{:#?}", args);

    //let needs_openssl = openssl_in_dependencies(args.work_dir.clone())?;
    //debug!("Needs OpenSSL: {}", needs_openssl);

    let work_dir = args
        .work_dir
        .unwrap_or(env::current_dir().expect("current directory"))
        .canonicalize()
        .expect("work dir canonicalizes");
    if !work_dir.exists() {
        return Err(ProgramError::WorkDirectoryDoesNotExist {
            loc: work_dir.to_string_lossy().as_ref().to_string(),
        })?;
    }
    if !work_dir.is_dir() {
        return Err(ProgramError::WorkDirectoryIsNotDirectory {
            loc: work_dir.to_string_lossy().as_ref().to_string(),
        })?;
    }

    // Process the dockerfile template.
    let v = args.rustup_version.unwrap_or(version_from_rustc()?);
    let dockerfile = match args.link_type {
        Link::Static => DockerStaticTemplate {
            rustup_version: &v,
            target_triple: &args.link_type.target_triple(),
            musl_version: &args.musl_version,
            openssl_version: &args.openssl_version,
        }.render()?,
        Link::Dynamic => DockerDynamicTemplate { rustup_version: &v }.render()?,
    };
    debug!("Dockerfile:\n{}", dockerfile);

    let user = get_user_by_uid(get_current_uid()).ok_or(ProgramError::GetCurrentUserFailed)?;
    debug!("User:\n{:#?}", user);

    let image_name = args.image_name.unwrap_or(
        (match args.link_type {
            Link::Static => "rust-amazonlinux-lambda-static",
            Link::Dynamic => "rust-amazonlinux-lambda-dynamic",
        }).to_string(),
    );

    let mut runner = DockerRunner::new(&dockerfile, &image_name, &user);
    runner.validate()?;
    runner.prepare_image()?;

    match args.cmd {
        Command::Build(x) => {
            commands::build::run(&x, &mut runner, &work_dir, &args.link_type.target_triple())?;
        }
        Command::Raw(x) => {
            commands::raw::run(&x, &mut runner, &work_dir)?;
        }
    }
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
