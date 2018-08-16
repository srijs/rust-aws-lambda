use cargo_metadata::{Metadata, Target};
use failure::Error;
use std::path::PathBuf;

#[derive(Debug, Fail)]
enum ManifestError {
    #[fail(display = "no binaries were found")]
    NoBinaries,
}

#[derive(Debug, Clone)]
pub struct ManifestInfo {
    pub binaries: Vec<Target>,
    pub source_location: PathBuf,
    pub target_location: PathBuf,
}

impl ManifestInfo {
    pub fn new(raw: Metadata) -> Result<Self, Error> {
        let binaries: Vec<Target> = raw
            .packages
            .iter()
            .flat_map(|p| p.targets.clone())
            .filter(|t| t.kind[0] == "bin")
            .collect();
        if binaries.len() < 1 {
            return Err(ManifestError::NoBinaries)?;
        }
        Ok(ManifestInfo {
            binaries,
            source_location: PathBuf::from(raw.workspace_root),
            target_location: PathBuf::from(raw.target_directory),
        })
    }
}
