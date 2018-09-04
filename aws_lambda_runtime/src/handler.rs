use std::marker::PhantomData;
use std::sync::Arc;

use failure::Error;
use futures::{future::FutureResult, Async, IntoFuture, Poll};
use tower_service::{NewService, Service};

/// Wrapper to convert a `Fn` into a `NewService`.
#[derive(Debug)]
pub struct Handler<F, R> {
    f: Arc<F>,
    _phan: PhantomData<fn() -> R>,
}

impl<F, R, S> From<F> for Handler<F, R>
where
    F: Fn(R) -> S,
    S: IntoFuture,
{
    fn from(f: F) -> Self {
        Handler {
            f: Arc::new(f),
            _phan: PhantomData,
        }
    }
}

impl<F, R, S> NewService for Handler<F, R>
where
    F: Fn(R) -> S,
    S: IntoFuture,
{
    type Request = R;
    type Response = S::Item;
    type Error = S::Error;
    type InitError = Error;
    type Future = FutureResult<Self::Service, Error>;
    type Service = HandlerService<F, R>;

    fn new_service(&self) -> FutureResult<Self::Service, Error> {
        Ok(HandlerService {
            f: self.f.clone(),
            _phan: PhantomData,
        }).into()
    }
}

#[derive(Debug)]
pub struct HandlerService<F, R> {
    f: Arc<F>,
    _phan: PhantomData<fn() -> R>,
}

impl<F, R, S> Service for HandlerService<F, R>
where
    F: Fn(R) -> S,
    S: IntoFuture,
{
    type Request = R;
    type Response = S::Item;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        (self.f)(req).into_future()
    }
}
