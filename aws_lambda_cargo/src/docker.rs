use askama::Template;
use failure::Error;
use rustc_version::Version;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use tempfile::tempdir;
use users::User;
use duct::cmd;

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
    pub rust_version: &'a String,
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
        let output = cmd("docker", vec!["--version"])
            .run()
            .map_err(|e| DockerError::BinaryNotFound { error: e })?;
        self.checked = true;
        if output.status.success() {
            Ok(())
        } else {
            Err(DockerError::VersionCommandFailed { status: output.status })?
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
        let child = cmd("docker", vec!["build",
            "-t", image_name,
            "-f", &file_path.to_string_lossy(),
            &dir.path().to_string_lossy()])
            .start()
            .map_err(|e| DockerError::ImageBuildCommandFailed { error: e })?;

        child
            .wait()
            .map_err(|e| DockerError::ImageBuildCommandFailed { error: e })?;

        info!("Successfully built docker image `{}`", image_name);
        Ok(String::from(image_name))
    }

    /// Run a command in docker.
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

        let location_in_docker = PathBuf::from("/code");
        let mut cache_location = source_location.clone();
        cache_location.push("./target/cached_data");

        let child = cmd("docker", vec!["run",
            // This makes the docker container stop after running.
            "--rm",
            // Speeds up builds by storing the cargo registry on the host.
            "-v", &format!("{}:/usr/local/cargo/registry", cache_location.to_string_lossy()),
            // Maps the source code into docker container.
            "-v", &format!("{}:/{}", source_location.to_string_lossy(), location_in_docker.to_string_lossy()),
            // Sets the working directory to where the code is in the container.
            "-w", &location_in_docker.to_string_lossy(),
            // Sets the proper user/group instead of the docker user/group.
            "-u", &format!("{}:{}", user.uid(), user.primary_group_id()),
            // The docker image to use for the container.
            &docker_image,
            // The (usually cargo) command to run.
            "bash", "-c", bash_command,
        ])
          .start()
          .map_err(|e| DockerError::ImageBuildCommandFailed { error: e })?;

        child
            .wait()
            .map_err(|e| DockerError::RunCommandReturnedError { error: e })?;

        Ok("my/coolbin.txt".to_string())
    }
}
