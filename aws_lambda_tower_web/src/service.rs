use failure::{Error, Fail};
use futures::{future::FutureResult, Async, Future, Poll};
use http;
use tower_service::{NewService, Service};

use aws_lambda_gateway::Body;

use tower_web::error::Catch;
use tower_web::routing::{Resource, RoutedService};
use tower_web::service::{NewWebService, WebService};
use tower_web::util::http::HttpMiddleware;
use tower_web::util::http::HttpService;
use tower_web::util::BufStream;

pub(crate) struct NewServiceWrapper<T: Resource, U, M> {
    pub(crate) inner: NewWebService<T, U, M>,
}

impl<T, U, M> NewService for NewServiceWrapper<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>, RequestBody = ::RequestBody>,
    M::Error: Fail,
    <M::ResponseBody as BufStream>::Error: Fail,
{
    type Request = http::Request<Body>;
    type Response = http::Response<Body>;
    type Error = Error;
    type Service = ServiceWrapper<T, U, M>;
    type InitError = Error;
    type Future = NewServiceWrapperFuture<T, U, M>;

    fn new_service(&self) -> Self::Future {
        NewServiceWrapperFuture {
            inner: self.inner.new_service(),
        }
    }
}

pub(crate) struct NewServiceWrapperFuture<
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>>,
> {
    inner: FutureResult<WebService<T, U, M>, <NewWebService<T, U, M> as NewService>::InitError>,
}

impl<T, U, M> Future for NewServiceWrapperFuture<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>, RequestBody = ::RequestBody>,
    M::Error: Fail,
    <M::ResponseBody as BufStream>::Error: Fail,
{
    type Item = ServiceWrapper<T, U, M>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(self.inner.poll()?.map(|inner| ServiceWrapper { inner }))
    }
}

pub(crate) struct ServiceWrapper<T: Resource, U: Catch, M: HttpMiddleware<RoutedService<T, U>>> {
    inner: WebService<T, U, M>,
}

impl<T, U, M> Service for ServiceWrapper<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>, RequestBody = ::RequestBody>,
    M::Error: Fail,
    <M::ResponseBody as BufStream>::Error: Fail,
{
    type Request = http::Request<Body>;
    type Response = http::Response<Body>;
    type Error = Error;
    type Future = ServiceWrapperFuture<T, U, M>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(self.inner.poll_ready()?)
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        let (parts, body) = request.into_parts();
        ServiceWrapperFuture::Waiting {
            future: self
                .inner
                .call(http::Request::from_parts(parts, ::RequestBody(Some(body)))),
        }
    }
}

pub(crate) enum ServiceWrapperFuture<T: Resource, U: Catch, M: HttpMiddleware<RoutedService<T, U>>>
{
    Waiting {
        future: <M::Service as HttpService>::Future,
    },
    Collecting {
        parts: Option<::http::response::Parts>,
        future: ::tower_web::util::buf_stream::Collect<M::ResponseBody, Vec<u8>>,
    },
}

impl<T, U, M> Future for ServiceWrapperFuture<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>, RequestBody = ::RequestBody>,
    M::Error: Fail,
    <M::ResponseBody as BufStream>::Error: Fail,
{
    type Item = http::Response<Body>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let next_state = match self {
            ServiceWrapperFuture::Waiting { ref mut future } => {
                let res = try_ready!(future.poll());
                let (parts, body) = res.into_parts();
                drop(future);
                ServiceWrapperFuture::Collecting {
                    parts: Some(parts),
                    future: body.collect(),
                }
            }
            ServiceWrapperFuture::Collecting {
                ref mut parts,
                ref mut future,
            } => {
                let body = try_ready!(future.poll()).into();
                let res = http::Response::from_parts(parts.take().unwrap(), body);
                return Ok(Async::Ready(res));
            }
        };
        *self = next_state;
        self.poll()
    }
}
