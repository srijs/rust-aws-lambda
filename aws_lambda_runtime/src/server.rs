use std::io;
use std::net::SocketAddr;

use failure::Error;
use futures::stream::FuturesUnordered;
use futures::{Async, Future, Poll, Sink, Stream};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_core::reactor::Handle;
use tokio_io::{io::ReadHalf, io::WriteHalf, AsyncRead, AsyncWrite};
use tower_service::{NewService, Service};
use void::Void;

use super::context::Context;
use super::error::{ConnectionError, RuntimeError};
use super::proto;

pub struct Server<S, I> {
    new_service: S,
    incoming: I,
    handle: Handle,
}

impl<S, I, Io> Server<S, I>
where
    S: NewService<Error = Error, InitError = Error> + 'static,
    S::Future: 'static,
    S::Service: 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    I: Stream<Item = (Io, SocketAddr), Error = io::Error> + 'static,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    pub fn new(new_service: S, incoming: I, handle: Handle) -> Server<S, I> {
        Server {
            new_service,
            incoming,
            handle,
        }
    }

    fn spawn_service(&mut self) -> impl Future<Item = S::Service, Error = ()> {
        self.new_service
            .new_service()
            .then(|service_result| match service_result {
                Ok(service) => Ok(service),
                Err(err) => {
                    error!("service error: {}", err);
                    Err(())
                }
            })
    }

    fn spawn(&mut self, stream: Io) -> Result<(), RuntimeError> {
        let connection = self.spawn_service().and_then(|service| {
            let connection = Connection::spawn(service, stream);
            connection.then(|res| {
                if let Err(err) = res {
                    error!("connection error: {}", err);
                }
                Ok(())
            })
        });
        self.handle.spawn(connection);

        Ok(())
    }
}

impl<S, I, Io> Future for Server<S, I>
where
    S: NewService<InitError = Error, Error = Error> + 'static,
    S::Service: 'static,
    S::Future: 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    I: Stream<Item = (Io, SocketAddr), Error = io::Error> + 'static,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    type Item = ();
    type Error = RuntimeError;

    fn poll(&mut self) -> Poll<(), RuntimeError> {
        loop {
            if let Some((stream, _)) =
                try_ready!(self.incoming.poll().map_err(RuntimeError::from_io))
            {
                self.spawn(stream)?;
            } else {
                return Ok(Async::Ready(()));
            }
        }
    }
}

struct Connection<S, Io>
where
    S: Service,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    service: S,
    decoder: proto::Decoder<ReadHalf<Io>, S::Request>,
    encoder: proto::Encoder<WriteHalf<Io>, S::Response>,
    futures: FuturesUnordered<Invocation<S>>,
}

impl<S, Io> Connection<S, Io>
where
    S: Service<Error = Error> + 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    fn spawn(service: S, io: Io) -> Self {
        let (r, w) = io.split();
        let decoder = proto::Decoder::new(r);
        let encoder = proto::Encoder::new(w);

        Connection {
            service,
            decoder,
            encoder,
            futures: FuturesUnordered::new(),
        }
    }

    fn poll_encoder(&mut self) -> Poll<(), ConnectionError> {
        Ok(self.encoder.poll_complete()?)
    }

    fn poll_futures(&mut self) -> Poll<(), ConnectionError> {
        loop {
            if let Some((seq, result)) = try_ready!(self.futures.poll()) {
                self.encoder
                    .start_send(proto::Response::Invoke(seq, result))?;
            } else {
                return Ok(Async::Ready(()));
            }
        }
    }

    fn poll_decoder(&mut self) -> Poll<(), ConnectionError> {
        loop {
            match self.decoder.poll() {
                Ok(Async::Ready(Some(request))) => match request {
                    proto::Request::Ping(seq) => {
                        self.encoder.start_send(proto::Response::Ping(seq))?;
                        continue;
                    }
                    proto::Request::Invoke(seq, _deadline, ctx, payload) => {
                        // TODO: enforce deadline
                        let future = ctx.with(|| self.service.call(payload));
                        self.futures.push(Invocation { seq, future, ctx });
                        continue;
                    }
                },
                Ok(Async::NotReady) => {
                    return Ok(Async::NotReady);
                }
                Ok(Async::Ready(None)) => {
                    return Ok(Async::Ready(()));
                }
                Err(proto::DecodeError::User(seq, err)) => {
                    self.encoder
                        .start_send(proto::Response::Invoke(seq, Err(err)))?;
                    continue;
                }
                Err(proto::DecodeError::Frame(err)) => {
                    return Err(err);
                }
            }
        }
    }
}

impl<S, Io> Future for Connection<S, Io>
where
    S: Service<Error = Error> + 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    type Item = ();
    type Error = ConnectionError;

    fn poll(&mut self) -> Poll<(), ConnectionError> {
        // poll the decoder first, as it may create work for futures and encoder
        let decoder_ready = self.poll_decoder()?.is_ready();
        // poll the futures next, as they might create work for the encoder
        let futures_ready = self.poll_futures()?.is_ready();
        // poll the encoder last, as it will never create other work
        let encoder_ready = self.poll_encoder()?.is_ready();

        if encoder_ready && futures_ready && decoder_ready {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}

struct Invocation<S: Service> {
    seq: u64,
    future: S::Future,
    ctx: Context,
}

impl<S> Future for Invocation<S>
where
    S: Service,
{
    type Item = (u64, Result<S::Response, S::Error>);
    type Error = Void;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let seq = self.seq;
        let future = &mut self.future;
        self.ctx.with(|| match future.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(res)) => Ok(Async::Ready((seq, Ok(res)))),
            Err(err) => Ok(Async::Ready((seq, Err(err)))),
        })
    }
}
