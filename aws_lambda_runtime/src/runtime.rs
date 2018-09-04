use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use failure::Error;
use futures::IntoFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::net::TcpListener;
use tokio::reactor::Handle;
use tokio::runtime::Runtime as TokioRuntime;
use tower_service::{NewService, Service};

use super::error::RuntimeError;
use super::handler::Handler;
use super::server::Server;

/// Runtime environment.
#[derive(Debug)]
pub struct Runtime {
    inner: TokioRuntime,
}

impl Runtime {
    /// Create a new `Runtime`, returning any error that happened during the creation.
    pub fn new() -> Result<Runtime, RuntimeError> {
        let inner = TokioRuntime::new().map_err(RuntimeError::from_io)?;
        Ok(Runtime { inner })
    }

    /// Retrieve a `Handle` to the underlying reactor.
    pub fn handle(&self) -> Handle {
        self.inner.reactor().clone()
    }

    /// Start the runtime with the given handler function.
    pub fn start<F, R, S>(self, f: F) -> Result<(), RuntimeError>
    where
        F: Fn(R) -> S + Send + Sync + 'static,
        S: IntoFuture<Error = Error> + Send,
        S::Future: Send,
        S::Item: Serialize + Send + 'static,
        R: DeserializeOwned + Send + 'static,
    {
        self.start_service(Handler::from(f))
    }

    /// Start the runtime with the given `Service`.
    pub fn start_service<S>(self, s: S) -> Result<(), RuntimeError>
    where
        S: NewService<InitError = Error, Error = Error> + Send + 'static,
        S::Service: Send + 'static,
        <S::Service as Service>::Future: Send,
        S::Future: Send + 'static,
        S::Request: DeserializeOwned + Send + Send + 'static,
        S::Response: Serialize + Send + 'static,
    {
        let port = server_port()?;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let listener = TcpListener::bind(&addr).map_err(RuntimeError::from_io)?;
        let server = Server::new(s, listener.incoming());
        self.inner.block_on_all(server)
    }
}

fn server_port() -> Result<u16, RuntimeError> {
    let reason = "the _LAMBDA_SERVER_PORT variable must specify a valid port to listen on";

    match env::var("_LAMBDA_SERVER_PORT") {
        Ok(var) => var.parse().map_err(|_| RuntimeError::environment(reason)),
        Err(_) => Err(RuntimeError::environment(reason)),
    }
}
