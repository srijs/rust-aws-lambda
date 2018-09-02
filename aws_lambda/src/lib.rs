extern crate failure;
extern crate futures;
extern crate log;
extern crate serde;

use failure::Error;
use futures::IntoFuture;
use serde::{de::DeserializeOwned, Serialize};

extern crate aws_lambda_events;
#[cfg(feature = "gateway")]
extern crate aws_lambda_gateway;
extern crate aws_lambda_runtime;

pub use aws_lambda_runtime::{Context, Runtime};

pub use aws_lambda_events::event;
pub use aws_lambda_runtime::context;
pub use aws_lambda_runtime::env;

#[cfg(feature = "gateway")]
pub mod gateway;
pub mod logger;

/// Start the lambda runtime using the provided handler function.
///
/// The function will block until the runtime shuts down or returns
/// with an error.
///
/// ## Example
///
/// ```no_run
/// extern crate aws_lambda as lambda;
///
/// fn main() {
///     lambda::start(|()| Ok("Hello ƛ!"))
/// }
/// ```
///
/// ## Panics
///
/// This function will panic if it fails to create the runtime.
///
/// If you wish to handle this case more, you can use `Runtime::start`
/// instead.
pub fn start<F, R, S>(f: F)
where
    F: Fn(R) -> S + 'static,
    S: IntoFuture<Error = Error>,
    S::Item: Serialize + Send + 'static,
    R: DeserializeOwned + Send + 'static,
{
    Runtime::new()
        .and_then(|runtime| runtime.start(f))
        .unwrap_or_else(|err| panic!("failed to start runtime: {}", err))
}
