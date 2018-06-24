use std::io;
use std::rc::Rc;

use failure::Error;
use futures::stream::FuturesUnordered;
use futures::{Async, Future, Poll, Sink, Stream};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Handle;
use tokio_io::{io::ReadHalf, io::WriteHalf, AsyncRead, AsyncWrite};
use tokio_service::{NewService, Service};
use void::Void;

use super::context::{self, Context};
use super::proto;

pub struct Server<S: NewService> {
    new_service: S,
    listener: TcpListener,
    handle: Handle,
}

impl<S> Server<S>
where
    S: NewService<Error = Error>,
    S::Instance: 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
{
    pub fn new(new_service: S, listener: TcpListener, handle: Handle) -> Server<S> {
        Server {
            new_service,
            listener,
            handle,
        }
    }

    fn spawn(&mut self, stream: TcpStream) -> Result<(), Error> {
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

impl<S> Future for Server<S>
where
    S: NewService<Error = Error>,
    S::Instance: 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        let accept_result = self.listener.accept();
        match accept_result {
            Ok((stream, _)) => {
                self.spawn(stream)?;
                self.poll()
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::WouldBlock {
                    Ok(Async::NotReady)
                } else {
                    Err(err.into())
                }
            }
        }
    }
}

pub struct Connection<S, Io>
where
    S: Service,
    Io: AsyncRead + AsyncWrite + Send + 'static,
{
    service: Rc<S>,
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
            service: Rc::new(service),
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
            match try_ready!(self.futures.poll()) {
                None => {
                    return Ok(Async::Ready(()));
                }
                Some((seq, result)) => {
                    self.encoder
                        .start_send(proto::Response::Invoke(seq, result))?;
                    continue;
                }
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
                        let service = self.service.clone();
                        let fut = Invocation::Starting(seq, service, payload, ctx);
                        self.futures.push(fut);
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

enum Invocation<S: Service> {
    Starting(u64, Rc<S>, S::Request, context::LambdaContext),
    Running(u64, S::Future),
    Swapping,
}

impl<S> Future for Invocation<S>
where
    S: Service,
{
    type Item = (u64, Result<S::Response, S::Error>);
    type Error = Void;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Invocation::Running(ref seq, ref mut future) = self {
            match future.poll() {
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Ok(Async::Ready(res)) => Ok(Async::Ready((*seq, Ok(res)))),
                Err(err) => Ok(Async::Ready((*seq, Err(err)))),
            }
        } else {
            match ::std::mem::replace(self, Invocation::Swapping) {
                Invocation::Starting(seq, svc, req, ctx) => {
                    Context::set_current(ctx);
                    *self = Invocation::Running(seq, svc.call(req));
                    self.poll()
                }
                _ => unreachable!(),
            }
        }
    }
}
