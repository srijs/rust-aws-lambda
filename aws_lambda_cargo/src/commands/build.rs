use failure::Error;
use progress::Progress;

arg_enum! {
    #[derive(Debug)]
    pub enum LinkType {
        Static,
        Dynamic,
    }
}

#[derive(Debug, StructOpt)]
pub struct Settings {
    #[structopt(long = "link_type", short = "l")]
    #[structopt(
        default_value = "Static",
        raw(
            possible_values = "&LinkType::variants()",
            case_insensitive = "true"
        )
    )]
    link: LinkType,
    #[structopt(name = "CARGO_OPTIONS")]
    /// Options to pass through to `cargo build`
    cargo_options: Vec<String>,
}

pub fn run(_progress: &mut Progress, _settings: &Settings) -> Result<(), Error> {
    trace!("Running `build` command");
    Ok(())
}
