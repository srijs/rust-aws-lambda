use std::io::{self, Read, Write};
use std::thread;

use failure::Error;
use futures::stream::FuturesUnordered;
use futures::sync::mpsc;
use futures::{Async, Future, Poll, Stream};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Handle;
use tokio_service::{NewService, Service};

use super::proto;

pub struct Server<S: NewService> {
    new_service: S,
    listener: TcpListener,
    handle: Handle,
    futures: FuturesUnordered<Box<Future<Item = (), Error = Error>>>,
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
            futures: FuturesUnordered::new(),
        }
    }

    fn spawn<R, W>(&mut self, r: R, w: W) -> Result<(), Error>
    where
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        let (req_send, req_recv) = mpsc::unbounded();
        let (res_send, res_recv) = mpsc::unbounded();
        let res_send_clone = res_send.clone();

        let service = self.new_service.new_service()?;

        thread::Builder::new()
            .name("lambda-send".to_owned())
            .spawn(|| res_loop(proto::Encoder::new(w), res_recv))?;
        thread::Builder::new()
            .name("lambda-recv".to_owned())
            .spawn(|| req_loop(proto::Decoder::new(r), req_send, res_send_clone))?;

        self.futures
            .push(svc_future(self.handle.clone(), service, req_recv, res_send));

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
        let futures_async = self.futures.poll()?;
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
                    match futures_async {
                        Async::Ready(None) => Ok(Async::NotReady),
                        Async::Ready(Some(_)) => self.poll(),
                        Async::NotReady => Ok(Async::NotReady),
                    }
                } else {
                    Err(err.into())
                }
            }
        }
    }
}

fn req_loop<R, T, U>(
    decoder: proto::Decoder<R, T>,
    req_sender: mpsc::UnboundedSender<proto::Request<T>>,
    res_sender: mpsc::UnboundedSender<proto::Response<U>>,
) where
    R: Read,
    T: DeserializeOwned,
{
    for result in decoder {
        match result {
            Ok(req) => {
                req_sender.unbounded_send(req).unwrap();
            }
            Err(proto::DecodeError::User(seq, err)) => {
                res_sender
                    .unbounded_send(proto::Response::Invoke(seq, Err(err)))
                    .unwrap();
            }
            Err(proto::DecodeError::Frame(err)) => {
                panic!("an error occurred during decoding: {}", err)
            }
        }
    }
}

fn res_loop<W, T>(
    mut encoder: proto::Encoder<W, T>,
    receiver: mpsc::UnboundedReceiver<proto::Response<T>>,
) where
    W: Write,
    T: Serialize,
{
    for res in receiver.wait() {
        encoder.encode(res.unwrap()).unwrap();
    }
}

fn svc_future<S>(
    handle: Handle,
    service: S,
    receiver: mpsc::UnboundedReceiver<proto::Request<S::Request>>,
    sender: mpsc::UnboundedSender<proto::Response<S::Response>>,
) -> Box<Future<Item = (), Error = Error>>
where
    S: Service<Error = Error> + 'static,
    S::Future: 'static,
    S::Request: 'static,
    S::Response: 'static,
{
    let fut = receiver
        .map_err(|()| panic!("receiver failed"))
        .for_each(move |req| {
            let cloned_sender = sender.clone();
            match req {
                proto::Request::Ping(seq) => {
                    cloned_sender
                        .unbounded_send(proto::Response::Ping(seq))
                        .unwrap();
                }
                proto::Request::Invoke(seq, deadline, ctx, payload) => {
                    handle.spawn(service.call(payload).then(move |result| {
                        cloned_sender
                            .unbounded_send(proto::Response::Invoke(seq, result))
                            .unwrap();
                        Ok(())
                    }));
                }
            }
            Ok(())
        });
    Box::new(fut)
}
