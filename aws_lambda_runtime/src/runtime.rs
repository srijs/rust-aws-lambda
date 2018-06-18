use std::env;
use std::io;
use std::marker::PhantomData;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::rc::Rc;

use failure::Error;
use futures::IntoFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Handle as OldHandle};
use tokio_reactor::Handle;
use tokio_service::{NewService, Service};

use super::server::Server;

/// Runtime environment.
#[derive(Debug)]
pub struct Runtime {
    core: Core,
}

impl Runtime {
    /// Create a new `Runtime`, returning any error that happened during the creation.
    pub fn new() -> Result<Runtime, Error> {
        let core = Core::new()?;
        Ok(Runtime { core })
    }

    /// Retrieve a `Handle` to the underlying reactor.
    pub fn handle(&self) -> Handle {
        self.core.handle().new_tokio_handle().clone()
    }

    /// Retrieve a `Handle` to the underlying reactor which cannot be sent across threads.
    ///
    /// This is useful for working with futures that rely on the old `tokio-core`
    /// crate rather than the new `tokio` or `tokio-reactor` crates.
    pub fn handle_old(&self) -> OldHandle {
        self.core.handle()
    }

    /// Start the runtime with the given handler function.
    pub fn start<F, R, S>(self, f: F) -> Result<(), Error>
    where
        F: Fn(R) -> S + 'static,
        S: IntoFuture<Error = Error>,
        S::Item: Serialize + Send + 'static,
        R: DeserializeOwned + Send + 'static,
    {
        self.start_service(ServiceFn {
            f: Rc::new(f),
            _phan: PhantomData,
        })
    }

    /// Start the runtime with the given `Service`.
    pub fn start_service<S>(mut self, s: S) -> Result<(), Error>
    where
        S: NewService<Error = Error>,
        S::Instance: 'static,
        S::Request: DeserializeOwned + Send + 'static,
        S::Response: Serialize + Send + 'static,
    {
        let port = env::var("_LAMBDA_SERVER_PORT")?.parse()?;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let handle = self.core.handle();
        let listener = TcpListener::bind(&addr, &handle)?;
        let server = Server::new(s, listener);
        self.core.run(server)?;
        Ok(())
    }
}

struct ServiceFn<F, R> {
    f: Rc<F>,
    _phan: PhantomData<fn() -> R>,
}

impl<F, R, S> Service for ServiceFn<F, R>
where
    F: Fn(R) -> S,
    S: IntoFuture,
{
    type Request = R;
    type Response = S::Item;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, req: Self::Request) -> Self::Future {
        (self.f)(req).into_future()
    }
}

impl<F, R, S> NewService for ServiceFn<F, R>
where
    F: Fn(R) -> S,
    S: IntoFuture,
{
    type Request = R;
    type Response = S::Item;
    type Error = S::Error;
    type Instance = Self;

    fn new_service(&self) -> Result<Self, io::Error> {
        Ok(ServiceFn {
            f: self.f.clone(),
            _phan: PhantomData,
        })
    }
}
