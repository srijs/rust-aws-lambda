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

mod docker;
mod templates;

use askama::Template;
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
}

arg_enum! {
    #[derive(Debug)]
    enum Link {
        Dynamic,
        Static,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo-lambda")]
#[structopt(raw(
    global_settings = "&[AppSettings::VersionlessSubcommands, AppSettings::InferSubcommands]"
))]
/// Easily use Rust with AWS Lambda
struct Cli {
    /// Link style to use.
    /// Statically linked binaries are larger but more compatible.
    #[structopt(
        name = "STYLE",
        short = "link",
        long = "link_style",
        default_value = "Dynamic",
        raw(
            possible_values = "&Link::variants()",
            case_insensitive = "true"
        )
    )]
    link_style: Link,
    #[structopt(name = "PATH", long = "manifest-path", parse(from_os_str))]
    manifest_path: Option<PathBuf>,
    #[structopt(long = "rustup_version", name = "RUSTUP VERSION")]
    rustup_version: Option<RustupVersion>,
    #[structopt(
        long = "musl_version",
        default_value = "1.1.19",
        name = "MUSL VERSION"
    )]
    musl_version: MuslVersion,
    #[structopt(
        long = "openssl_version",
        default_value = "1.1.0i",
        name = "OPENSSL VERSION"
    )]
    openssl_version: OpenSslVersion,
    #[structopt(long = "image_name", name = "DOCKER IMAGE NAME")]
    image_name: Option<String>,
    #[structopt(name = "CARGO COMMAND")]
    cargo_command: Vec<String>,
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

fn inner_main(args: Cli) -> Result<(), Error> {
    debug!("Arguments:\n{:#?}", args);

    // Process the dockerfile template.
    let v = args.rustup_version.unwrap_or(version_from_rustc()?);
    let dockerfile = match args.link_style {
        Link::Static => DockerStaticTemplate {
            rustup_version: &v,
            build_target: "x86_64-unknown-linux-musl",
            musl_version: &args.musl_version,
            openssl_version: &args.openssl_version,
        }.render()?,
        Link::Dynamic => DockerDynamicTemplate { rustup_version: &v }.render()?,
    };
    debug!("Dockerfile:\n{}", dockerfile);

    let user = get_user_by_uid(get_current_uid()).ok_or(ProgramError::GetCurrentUserFailed)?;
    debug!("User:\n{:#?}", user);

    let image_name = args.image_name.unwrap_or(
        (match args.link_style {
            Link::Static => "rust-amazonlinux-lambda-static",
            Link::Dynamic => "rust-amazonlinux-lambda-dynamic",
        }).to_string(),
    );

    let mut runner = DockerRunner::new(&dockerfile, &image_name);
    runner.validate()?;
    let docker_image = runner.prepare_image()?;
    runner.run(
        &args.cargo_command,
        &docker_image,
        &args
            .manifest_path
            .unwrap_or(env::current_dir().expect("working directory")),
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
