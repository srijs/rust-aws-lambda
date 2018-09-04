use std::io;

use bytes::Buf;
use futures::{Async, Poll};
use void::Void;

use aws_lambda_gateway::Body;

use tower_web::util::BufStream;

pub struct RequestBody(pub(crate) Option<Body>);

impl BufStream for RequestBody {
    type Item = RequestBodyBuf;
    type Error = Void;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(
            self.0
                .take()
                .map(|body| RequestBodyBuf(io::Cursor::new(body))),
        ))
    }
}

pub struct RequestBodyBuf(io::Cursor<Body>);

impl Buf for RequestBodyBuf {
    fn remaining(&self) -> usize {
        self.0.remaining()
    }

    fn bytes(&self) -> &[u8] {
        self.0.bytes()
    }

    fn advance(&mut self, cnt: usize) {
        self.0.advance(cnt)
    }
}
