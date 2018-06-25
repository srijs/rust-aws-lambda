use std::io;
use std::net::SocketAddr;

use failure::Error;
use futures::stream::FuturesUnordered;
use futures::{Async, Future, Poll, Sink, Stream};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_core::reactor::Handle;
use tokio_io::{io::ReadHalf, io::WriteHalf, AsyncRead, AsyncWrite};
use tokio_service::{NewService, Service};
use void::Void;

use super::context::Context;
use super::proto;

pub struct Server<S, I> {
    new_service: S,
    incoming: I,
    handle: Handle,
}

impl<S, I, Io> Server<S, I>
where
    S: NewService<Error = Error>,
    S::Instance: 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    I: Stream<Item = (Io, SocketAddr), Error = io::Error>,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    pub fn new(new_service: S, incoming: I, handle: Handle) -> Server<S, I> {
        Server {
            new_service,
            incoming,
            handle,
        }
    }

    fn spawn(&mut self, stream: Io) -> Result<(), Error> {
        let service = self.new_service.new_service()?;
        let connection = Connection::spawn(service, stream)?;
        self.handle.spawn(connection.then(|res| {
            if let Err(err) = res {
                error!("connection error: {}", err);
            }
            Ok(())
        }));

        Ok(())
    }
}

impl<S, I, Io> Future for Server<S, I>
where
    S: NewService<Error = Error>,
    S::Instance: 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    I: Stream<Item = (Io, SocketAddr), Error = io::Error>,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        loop {
            if let Some((stream, _)) = try_ready!(self.incoming.poll()) {
                self.spawn(stream)?;
            } else {
                return Ok(Async::Ready(()));
            }
        }
    }
}

pub struct Connection<S, Io>
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
    fn spawn(service: S, io: Io) -> Result<Self, Error> {
        let (r, w) = io.split();
        let decoder = proto::Decoder::new(r);
        let encoder = proto::Encoder::new(w)?;

        Ok(Connection {
            service,
            decoder,
            encoder,
            futures: FuturesUnordered::new(),
        })
    }

    fn poll_encoder(&mut self) -> Poll<(), Error> {
        Ok(self.encoder.poll_complete()?)
    }

    fn poll_futures(&mut self) -> Poll<(), Error> {
        loop {
            if let Some((seq, result)) = try_ready!(self.futures.poll()) {
                self.encoder
                    .start_send(proto::Response::Invoke(seq, result))?;
            } else {
                return Ok(Async::Ready(()));
            }
        }
    }

    fn poll_decoder(&mut self) -> Poll<(), Error> {
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
                    bail!("an error occurred during decoding: {}", err)
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
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
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
