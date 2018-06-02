use std::any::Any;
use std::io;
use std::thread;

use futures::sync::oneshot;
use futures::{Future, Poll};

pub struct AsyncThread<T> {
    receiver: oneshot::Receiver<T>,
    join: Option<thread::JoinHandle<()>>,
}

impl<T> Future for AsyncThread<T> {
    type Item = T;
    type Error = Box<Any + Send + 'static>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.receiver.poll().map_err(|_| {
            if let Some(join) = self.join.take() {
                join.join().err().unwrap()
            } else {
                panic!("AsyncThreadHandle polled after ready")
            }
        })
    }
}

pub trait AsyncThreadExt {
    fn spawn_async<F, T>(self, f: F) -> io::Result<AsyncThread<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static;
}

impl AsyncThreadExt for thread::Builder {
    fn spawn_async<F, T>(self, f: F) -> io::Result<AsyncThread<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let (sender, receiver) = oneshot::channel();
        let join = self.spawn(move || {
            sender.send(f()).ok();
        })?;
        Ok(AsyncThread {
            receiver,
            join: Some(join),
        })
    }
}
