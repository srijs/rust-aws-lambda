extern crate base64;
extern crate failure;
#[macro_use]
extern crate futures;
pub extern crate http;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_service;

use std::io;
use std::sync::Arc;

use failure::Error;
use futures::{Async, Future, IntoFuture, Poll};
use http::{Request, Response};
use tokio_service::{NewService, Service};

mod body;
pub use body::Body;
mod request;
pub use request::ApiGatewayProxyRequest;
mod response;
pub use response::ApiGatewayProxyResponse;

#[derive(Clone)]
struct Handler {
    inner: Arc<Fn(Request<Body>) -> Box<Future<Item = Response<Body>, Error = Error>>>,
}

impl Handler {
    fn new<F, R>(f: F) -> Handler
    where
        F: Fn(Request<Body>) -> R + 'static,
        R: IntoFuture<Item = Response<Body>, Error = Error>,
        R::Future: Sized + 'static,
    {
        Handler {
            inner: Arc::new(move |req| Box::new(f(req).into_future())),
        }
    }
}

pub struct NewApiGatewayProxy {
    handler: Handler,
}

impl NewApiGatewayProxy {
    pub fn new_with_handler<F, R>(f: F) -> NewApiGatewayProxy
    where
        F: Fn(Request<Body>) -> R + 'static,
        R: IntoFuture<Item = Response<Body>, Error = Error>,
        R::Future: Sized + 'static,
    {
        NewApiGatewayProxy {
            handler: Handler::new(f),
        }
    }
}

impl NewService for NewApiGatewayProxy where {
    type Error = Error;
    type Request = ApiGatewayProxyRequest;
    type Response = ApiGatewayProxyResponse;
    type Instance = ApiGatewayProxy;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(ApiGatewayProxy {
            handler: self.handler.clone(),
        })
    }
}

pub struct ApiGatewayProxy {
    handler: Handler,
}

impl Service for ApiGatewayProxy {
    type Error = Error;
    type Request = ApiGatewayProxyRequest;
    type Response = ApiGatewayProxyResponse;
    type Future = ApiGatewayProxyFuture;

    fn call(&self, req: Self::Request) -> Self::Future {
        let inner = (self.handler.inner)(req.0);
        ApiGatewayProxyFuture { inner }
    }
}

pub struct ApiGatewayProxyFuture {
    inner: Box<Future<Item = Response<Body>, Error = Error>>,
}

impl Future for ApiGatewayProxyFuture {
    type Item = ApiGatewayProxyResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let http_res = try_ready!(self.inner.poll());
        Ok(Async::Ready(ApiGatewayProxyResponse(http_res)))
    }
}
