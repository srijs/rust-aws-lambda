#[macro_use]
extern crate failure;
#[macro_use]
extern crate futures;
extern crate gob;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_schema;
#[macro_use]
extern crate serde_schema_derive;
extern crate tokio_core;
extern crate tokio_service;

use futures::IntoFuture;
use failure::Error;
use serde::{Serialize, de::DeserializeOwned};

pub mod context;
mod proto;
mod runtime;
mod server;

pub use context::Context;
pub use runtime::Runtime;

pub fn start<F, R, S>(f: F)
where
    F: Fn(R) -> S + 'static,
    S: IntoFuture<Error = Error>,
    S::Item: Serialize + Send + 'static,
    R: DeserializeOwned + Send + 'static,
{
    Runtime::new().unwrap().start(f).unwrap()
}
