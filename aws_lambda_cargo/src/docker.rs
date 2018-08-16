use askama::Template;
use failure::Error;
use rustc_version::Version;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use tempfile::tempdir;
use users::User;

#[derive(Debug, Fail)]
enum DockerError {
    #[fail(display = "docker is not installed or running: {}", error)]
    BinaryNotFound { error: io::Error },
    #[fail(
        display = "unable to create directory for dockerfile: {}",
        error
    )]
    DockerfileTempLocationFailed { error: io::Error },
    #[fail(display = "unable to create dockerfile: {}", error)]
    DockerfileCreateFailed { error: io::Error },
    #[fail(display = "unable to write dockerfile: {}", error)]
    DockerfileWriteFailed { error: io::Error },
    #[fail(display = "`docker --version` failed: {}", status)]
    VersionCommandFailed { status: ExitStatus },
    #[fail(display = "unable to build docker image: {}", error)]
    ImageBuildCommandFailed { error: io::Error },
    #[fail(display = "unable to run command in container: {}", error)]
    RunCommandFailed { error: io::Error },
    #[fail(
        display = "command run in container returned error: {}",
        error
    )]
    RunCommandReturnedError { error: io::Error },
}

#[derive(Template)]
#[template(path = "Dockerfile.dynamic")]
pub struct DockerDynamicTemplate<'a> {
    pub rust_version: &'a Version,
}

pub struct DockerRunner {
    dockerfile: String,
    checked: bool,
}

impl DockerRunner {
    pub fn new(dockerfile: String) -> Self {
        DockerRunner {
            checked: false,
            dockerfile,
        }
    }
    pub fn validate(&mut self) -> Result<(), Error> {
        trace!("Validating docker install");
        let status = Command::new("docker")
            .args(vec!["--version"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|e| DockerError::BinaryNotFound { error: e })?;
        self.checked = true;
        if status.success() {
            Ok(())
        } else {
            Err(DockerError::VersionCommandFailed { status })?
        }
    }

    /// Make or update the docker image. This is idempotent.
    pub fn prepare_image(&mut self) -> Result<String, Error> {
        trace!("Making docker image");
        if !self.checked {
            self.validate()?;
        }

        let image_name = "rust-amazonlinux";

        // Write Dockerfile to a temporary location. This doesn't cause a
        // rebuild every time because docker is smart enough to hash the
        // contents.
        let dir = tempdir().map_err(|e| DockerError::DockerfileTempLocationFailed { error: e })?;
        let file_path = dir.path().join("Dockerfile");
        let mut file = File::create(file_path.clone())
            .map_err(|e| DockerError::DockerfileCreateFailed { error: e })?;
        file.write_all(self.dockerfile.as_bytes())
            .map_err(|e| DockerError::DockerfileWriteFailed { error: e })?;

        // Create or update the docker image.
        let mut child = Command::new("docker")
            .args(vec!["build",
            "-t", image_name,
            "-f", &file_path.to_string_lossy(),
            &dir.path().to_string_lossy()])
            //.stdout(Stdio::null())
            //.stderr(Stdio::null())
            .spawn()
            .map_err(|e| DockerError::ImageBuildCommandFailed { error: e })?;

        child
            .wait()
            .map_err(|e| DockerError::ImageBuildCommandFailed { error: e })?;

        info!("Successfully built docker image `{}`", image_name);
        Ok(String::from(image_name))
    }

    /// Make or update the docker image. This is idempotent.
    pub fn run(
        &mut self,
        bash_command: &str,
        docker_image: &str,
        source_location: &PathBuf,
        user: &User,
    ) -> Result<String, Error> {
        trace!("Running command in docker image");
        if !self.checked {
            self.validate()?;
        }

        let location_in_docker = "/code";

        let mut child = Command::new("docker").arg("run").arg("--rm")
        .args(vec![
            "-v", &format!("{}:/{}", source_location.to_string_lossy(), location_in_docker),
            "-w", location_in_docker,
            "-u", &format!("{}:{}", user.uid(), user.primary_group_id()),
            &docker_image,
            "bash", "-c", bash_command,
        ])
        //.stdout(Stdio::null())
        //.stderr(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| DockerError::RunCommandFailed { error: e })?;

        child
            .wait()
            .map_err(|e| DockerError::RunCommandReturnedError { error: e })?;

        Ok("my/coolbin.txt".to_string())
    }
}
