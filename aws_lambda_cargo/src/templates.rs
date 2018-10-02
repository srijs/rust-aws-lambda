use askama::Template;
use std::{fmt, str};

#[derive(Debug)]
pub struct MuslVersion(String);
impl fmt::Display for MuslVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl str::FromStr for MuslVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MuslVersion(s.to_string()))
    }
}

#[derive(Debug)]
pub struct OpenSslVersion(String);
impl fmt::Display for OpenSslVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl str::FromStr for OpenSslVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OpenSslVersion(s.to_string()))
    }
}

#[derive(Debug)]
pub struct RustupVersion(String);
impl fmt::Display for RustupVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl str::FromStr for RustupVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RustupVersion(s.to_string()))
    }
}
impl RustupVersion {
    pub fn new(s: &str) -> Self {
        RustupVersion(s.to_string())
    }
}

#[derive(Template)]
#[template(path = "Dockerfile.dynamic")]
pub struct DockerDynamicTemplate<'a> {
    pub rustup_version: &'a RustupVersion,
}

#[derive(Template)]
#[template(path = "Dockerfile.static")]
pub struct DockerStaticTemplate<'a> {
    pub rustup_version: &'a RustupVersion,
    pub target_triple: &'a str,
    pub musl_version: &'a MuslVersion,
    pub openssl_version: &'a OpenSslVersion,
}
