use std::any::Any;
use std::io::{self, Read, Write};
use std::rc::Rc;
use std::thread;

use failure::Error;
use futures::future::lazy;
use futures::stream::FuturesUnordered;
use futures::sync::mpsc;
use futures::{Async, Future, Poll, Sink, Stream};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Handle;
use tokio_service::{NewService, Service};

use super::context::Context;
use super::proto;
use super::utils::{AsyncThread, AsyncThreadExt};

const REQ_QUEUE_MAX: usize = 128;

pub struct Server<S: NewService> {
    new_service: S,
    listener: TcpListener,
    handle: Handle,
    connections: FuturesUnordered<Connection>,
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

    fn spawn<R, W>(&mut self, r: R, w: W) -> Result<(), Error>
    where
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        let service = self.new_service.new_service()?;
        let connection = Connection::spawn(service, self.handle.clone(), r, w)?;
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
            Ok((mut stream, _)) => {
                stream.set_nonblocking(false)?;
                let cloned_stream = stream.try_clone()?;
                self.spawn(stream, cloned_stream)?;
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

pub struct Connection {
    send_thread: AsyncThread<Result<(), Error>>,
    recv_thread: AsyncThread<Result<(), Error>>,
    service_future: Box<Future<Item = (), Error = Error>>,
}

impl Connection {
    fn spawn<S, R, W>(service: S, handle: Handle, r: R, w: W) -> Result<Connection, Error>
    where
        S: Service<Error = Error> + 'static,
        S::Request: DeserializeOwned + Send + 'static,
        S::Response: Serialize + Send + 'static,
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        let (req_send, req_recv) = mpsc::channel(REQ_QUEUE_MAX);
        let (res_send, res_recv) = mpsc::unbounded();
        let res_send_clone = res_send.clone();

        let send_thread = thread::Builder::new()
            .name("lambda-send".to_owned())
            .spawn_async(|| res_loop(w, res_recv))?;
        let recv_thread = thread::Builder::new()
            .name("lambda-recv".to_owned())
            .spawn_async(|| req_loop(r, req_send, res_send_clone))?;
        let service_future = svc_future(handle, service, req_recv, res_send);

        Ok(Connection {
            send_thread,
            recv_thread,
            service_future,
        })
    }
}

fn poll_thread(poll: Poll<Result<(), Error>, Box<Any + Send + 'static>>) -> Poll<(), Error> {
    match poll {
        Err(panic) => Err(format_err!("thread panicked")),
        Ok(Async::Ready(Err(err))) => Err(err),
        Ok(Async::Ready(Ok(()))) => Ok(Async::Ready(())),
        Ok(Async::NotReady) => Ok(Async::NotReady),
    }
}

impl Future for Connection {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        let recv_ready = poll_thread(self.recv_thread.poll())?.is_ready();
        let send_ready = poll_thread(self.send_thread.poll())?.is_ready();
        if recv_ready && send_ready {
            Ok(Async::Ready(()))
        } else {
            self.service_future.poll()
        }
    }
}

fn req_loop<R, T, U>(
    r: R,
    req_sender: mpsc::Sender<proto::Request<T>>,
    res_sender: mpsc::UnboundedSender<proto::Response<U>>,
) -> Result<(), Error>
where
    R: Read,
    T: DeserializeOwned,
{
    let mut blocking_req_sender = req_sender.wait();
    let mut decoder = proto::Decoder::new(r);
    for result in decoder {
        match result {
            Ok(req) => {
                if blocking_req_sender.send(req).is_err() {
                    return Ok(());
                }
            }
            Err(proto::DecodeError::User(seq, err)) => {
                if res_sender
                    .unbounded_send(proto::Response::Invoke(seq, Err(err)))
                    .is_err()
                {
                    return Ok(());
                }
            }
            Err(proto::DecodeError::Frame(err)) => {
                bail!("an error occurred during decoding: {}", err)
            }
        }
    }
    Ok(())
}

fn res_loop<W, T>(w: W, receiver: mpsc::UnboundedReceiver<proto::Response<T>>) -> Result<(), Error>
where
    W: Write,
    T: Serialize,
{
    let mut encoder = proto::Encoder::new(w)?;
    for result in receiver.wait() {
        if let Ok(response) = result {
            encoder.encode(response)?;
        } else {
            return Ok(());
        }
    }
    Ok(())
}

fn svc_future<S>(
    handle: Handle,
    service: S,
    receiver: mpsc::Receiver<proto::Request<S::Request>>,
    sender: mpsc::UnboundedSender<proto::Response<S::Response>>,
) -> Box<Future<Item = (), Error = Error>>
where
    S: Service<Error = Error> + 'static,
    S::Future: 'static,
    S::Request: 'static,
    S::Response: 'static,
{
    let rc_service = Rc::new(service);
    let fut = receiver
        .then(move |result| {
            let cloned_sender = sender.clone();
            match result {
                Ok(proto::Request::Ping(seq)) => {
                    // ignore if sending fails
                    cloned_sender
                        .unbounded_send(proto::Response::Ping(seq))
                        .ok();
                }
                Ok(proto::Request::Invoke(seq, deadline, ctx, payload)) => {
                    let rc_service_clone = rc_service.clone();
                    let invoke_fut = lazy(move || {
                        Context::set_current(ctx);
                        rc_service_clone.call(payload).then(move |result| {
                            // ignore if sending fails
                            cloned_sender
                                .unbounded_send(proto::Response::Invoke(seq, result))
                                .ok();
                            Ok(())
                        })
                    });
                    handle.spawn(invoke_fut);
                }
                Err(_) => {}
            }
            Ok(())
        })
        .for_each(|_| Ok(()));
    Box::new(fut)
}
