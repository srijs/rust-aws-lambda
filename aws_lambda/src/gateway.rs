use failure::Error;
use futures::IntoFuture;

use aws_lambda_gateway::NewApiGatewayProxy;
use aws_lambda_runtime::Handler;

pub use aws_lambda_gateway::{http, Body};

pub type Request = http::Request<Body>;
pub type Response = http::Response<Body>;

/// Start the lambda gateway runtime using the provided handler function.
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
///     lambda::gateway::start(|req| {
///         let res = lambda::gateway::response()
///             .status(200)
///             .body("Hello ƛ!".into())?;
///         Ok(res)
///     })
/// }
/// ```
///
/// ## Panics
///
/// This function will panic if it fails to create the runtime.
pub fn start<F, S>(f: F)
where
    F: Fn(Request) -> S + Send + Sync + 'static,
    S: IntoFuture<Item = Response, Error = Error>,
    S::Future: Send + 'static,
{
    let service = NewApiGatewayProxy::new(Handler::from(f));
    ::Runtime::new()
        .and_then(|runtime| runtime.start_service(service))
        .unwrap_or_else(|err| panic!("failed to start runtime: {}", err))
}

pub fn response() -> http::response::Builder {
    http::response::Builder::new()
}
