use std::io;
use std::rc::Rc;

use failure::Error;
use futures::stream::FuturesUnordered;
use futures::{Async, Future, Poll, Sink, Stream};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_service::{NewService, Service};
use void::Void;

use super::context::{self, Context};
use super::proto;

pub struct Server<S: NewService> {
    new_service: S,
    listener: TcpListener,
    handle: Handle,
    connections: FuturesUnordered<Connection<S::Instance, TcpStream, TcpStream>>,
}

impl<S> Server<S>
where
    S: NewService<Error = Error>,
    S::Instance: 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
{
    pub fn new(new_service: S, handle: Handle, listener: TcpListener) -> Server<S> {
        Server {
            new_service,
            listener,
            handle,
            connections: FuturesUnordered::new(),
        }
    }

    fn spawn(&mut self, r: TcpStream, w: TcpStream) -> Result<(), Error> {
        let service = self.new_service.new_service()?;
        let connection = Connection::spawn(service, r, w)?;
        self.connections.push(connection);

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
        let connections_poll = self.connections.poll();
        let accept_result = self.listener.accept_std();
        match accept_result {
            Ok((stream, _)) => {
                let cloned_stream = stream.try_clone()?;
                let handle = self.handle.clone();
                self.spawn(
                    TcpStream::from_stream(stream, &handle)?,
                    TcpStream::from_stream(cloned_stream, &handle)?,
                )?;
                self.poll()
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::WouldBlock {
                    match connections_poll {
                        Ok(Async::Ready(None)) => Ok(Async::NotReady),
                        Ok(Async::Ready(Some(_))) => self.poll(),
                        Ok(Async::NotReady) => Ok(Async::NotReady),
                        Err(_err) => {
                            // TODO: log connection error
                            self.poll()
                        }
                    }
                } else {
                    Err(err.into())
                }
            }
        }
    }
}

pub struct Connection<S, R, W>
where
    S: Service,
    R: AsyncRead + Send + 'static,
    W: AsyncWrite + Send + 'static,
{
    service: Rc<S>,
    decoder: ::futures::stream::Fuse<proto::Decoder<R, S::Request>>,
    encoder: proto::Encoder<W, S::Response>,
    futures: FuturesUnordered<Invocation<S>>,
}

impl<S, R, W> Connection<S, R, W>
where
    S: Service<Error = Error> + 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    R: AsyncRead + Send + 'static,
    W: AsyncWrite + Send + 'static,
{
    fn spawn(service: S, r: R, w: W) -> Result<Self, Error> {
        let decoder = proto::Decoder::new(r).fuse();
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
        match try_ready!(self.futures.poll()) {
            None => Ok(Async::Ready(())),
            Some((seq, result)) => {
                self.encoder
                    .start_send(proto::Response::Invoke(seq, result))?;
                self.poll()
            }
        }
    }

    fn poll_decoder(&mut self) -> Poll<(), Error> {
        match self.decoder.poll() {
            Ok(Async::Ready(Some(request))) => match request {
                proto::Request::Ping(seq) => {
                    self.encoder.start_send(proto::Response::Ping(seq))?;
                    self.poll()
                }
                proto::Request::Invoke(seq, deadline, ctx, payload) => {
                    let service = self.service.clone();
                    let fut = Invocation::Starting(seq, service, payload, ctx);
                    self.futures.push(fut);
                    self.poll()
                }
            },
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(None)) => Ok(Async::Ready(())),
            Err(proto::DecodeError::User(seq, err)) => {
                self.encoder
                    .start_send(proto::Response::Invoke(seq, Err(err)))?;
                self.poll()
            }
            Err(proto::DecodeError::Frame(err)) => {
                bail!("an error occurred during decoding: {}", err)
            }
        }
    }
}

impl<S, R, W> Future for Connection<S, R, W>
where
    S: Service<Error = Error> + 'static,
    S::Request: DeserializeOwned + Send + 'static,
    S::Response: Serialize + Send + 'static,
    R: AsyncRead + Send + 'static,
    W: AsyncWrite + Send + 'static,
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        let decoder_ready = self.poll_decoder()?.is_ready();
        let futures_ready = self.poll_futures()?.is_ready();
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
        match ::std::mem::replace(self, Invocation::Swapping) {
            Invocation::Starting(seq, svc, req, ctx) => {
                Context::set_current(ctx);
                *self = Invocation::Running(seq, svc.call(req));
                self.poll()
            }
            Invocation::Running(seq, mut future) => match future.poll() {
                Ok(Async::NotReady) => {
                    *self = Invocation::Running(seq, future);
                    Ok(Async::NotReady)
                }
                Ok(Async::Ready(res)) => Ok(Async::Ready((seq, Ok(res)))),
                Err(err) => Ok(Async::Ready((seq, Err(err)))),
            },
            Invocation::Swapping => panic!("Invocation polled after ready"),
        }
    }
}
