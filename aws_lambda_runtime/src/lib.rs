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

pub mod context;
mod proto;
pub mod runtime;
mod server;

pub use context::Context;
pub use runtime::Runtime;
pub use runtime::start;
