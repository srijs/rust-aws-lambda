extern crate base64;
extern crate failure;
#[macro_use]
extern crate futures;
pub extern crate http;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tower_service;

use failure::Error;
use futures::{Async, Future, Poll};
use http::{Request, Response};
use tower_service::{NewService, Service};

mod body;
pub use body::Body;
mod request;
pub use request::PathParameters;
pub use request::QueryParameters;
pub use request::ApiGatewayProxyRequest;
mod response;
pub use response::ApiGatewayProxyResponse;

pub struct NewApiGatewayProxy<S> {
    new_service: S,
}

impl<S> NewApiGatewayProxy<S>
where
    S: NewService<Error = Error, Request = Request<Body>, Response = Response<Body>>,
{
    pub fn new(new_service: S) -> NewApiGatewayProxy<S> {
        NewApiGatewayProxy { new_service }
    }
}

impl<S> NewService for NewApiGatewayProxy<S>
where
    S: NewService<Error = Error, Request = Request<Body>, Response = Response<Body>>,
{
    type Future = NewApiGatewayProxyFuture<S>;
    type InitError = S::InitError;

    type Service = ApiGatewayProxy<S::Service>;
    type Request = ApiGatewayProxyRequest;
    type Response = ApiGatewayProxyResponse;
    type Error = Error;

    fn new_service(&self) -> Self::Future {
        NewApiGatewayProxyFuture(self.new_service.new_service())
    }
}

pub struct ApiGatewayProxy<S> {
    service: S,
}

impl<S> Service for ApiGatewayProxy<S>
where
    S: Service<Error = Error, Request = Request<Body>, Response = Response<Body>>,
{
    type Error = Error;
    type Request = ApiGatewayProxyRequest;
    type Response = ApiGatewayProxyResponse;
    type Future = ApiGatewayProxyFuture<S>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let inner = self.service.call(req.0);
        ApiGatewayProxyFuture { inner }
    }
}

pub struct NewApiGatewayProxyFuture<S: NewService>(S::Future);

impl<S: NewService> Future for NewApiGatewayProxyFuture<S> {
    type Item = ApiGatewayProxy<S::Service>;
    type Error = S::InitError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let service = try_ready!(self.0.poll());
        Ok(Async::Ready(ApiGatewayProxy { service }))
    }
}

pub struct ApiGatewayProxyFuture<S: Service> {
    inner: S::Future,
}

impl<S> Future for ApiGatewayProxyFuture<S>
where
    S: Service<Error = Error, Request = Request<Body>, Response = Response<Body>>,
{
    type Item = ApiGatewayProxyResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let http_res = try_ready!(self.inner.poll());
        Ok(Async::Ready(ApiGatewayProxyResponse(http_res)))
    }
}
