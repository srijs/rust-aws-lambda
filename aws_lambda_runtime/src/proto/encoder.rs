use std::io::Cursor;
use std::marker::PhantomData;

use bytes::Buf;
use failure::Error;
use futures::{Async, AsyncSink, Poll, Sink, StartSend};
use gob::{ser::TypeId, StreamSerializer};
use serde::Serialize;
use serde_bytes::Bytes;
use serde_schema::SchemaSerialize;
use tokio_io::AsyncWrite;

use super::messages;

#[derive(Serialize, SchemaSerialize)]
#[serde(rename = "Response")]
struct RpcResponse {
    #[serde(rename = "ServiceMethod")]
    service_method: &'static str,
    #[serde(rename = "Seq")]
    seq: u64,
    #[serde(rename = "Error")]
    error: Option<String>,
}

pub enum Response<T> {
    Ping(u64),
    Invoke(u64, Result<T, Error>),
}

pub struct Encoder<W, T>
where
    W: AsyncWrite,
{
    write: W,
    flushing: Cursor<Vec<u8>>,
    payload_buf: Vec<u8>,
    stream: StreamSerializer<Vec<u8>>,
    _phan: PhantomData<T>,
    type_id_response: TypeId,
    type_id_ping_response: TypeId,
    type_id_invoke_response: TypeId,
}

impl<W, T> Encoder<W, T>
where
    W: AsyncWrite,
{
    pub fn new(w: W) -> Result<Encoder<W, T>, Error> {
        let stream_buf = Vec::with_capacity(4096);
        let flush_buf = Vec::with_capacity(4096);
        let payload_buf = Vec::with_capacity(4096);

        let mut stream = StreamSerializer::new(stream_buf);
        let type_id_response = RpcResponse::schema_register(stream.schema_mut())?;
        let type_id_ping_response = messages::PingResponse::schema_register(stream.schema_mut())?;
        let type_id_invoke_response =
            messages::InvokeResponse::schema_register(stream.schema_mut())?;
        Ok(Encoder {
            write: w,
            flushing: Cursor::new(flush_buf),
            payload_buf,
            stream,
            type_id_response,
            type_id_ping_response,
            type_id_invoke_response,
            _phan: PhantomData,
        })
    }
}

impl<W, T> Sink for Encoder<W, T>
where
    W: AsyncWrite,
    T: Serialize,
{
    type SinkItem = Response<T>;
    type SinkError = Error;

    fn start_send(&mut self, res: Response<T>) -> StartSend<Self::SinkItem, Self::SinkError> {
        match res {
            Response::Ping(seq) => {
                self.stream.serialize_with_type_id(
                    self.type_id_response,
                    &RpcResponse {
                        service_method: "Function.Ping",
                        seq: seq,
                        error: None,
                    },
                )?;
                self.stream.serialize_with_type_id(
                    self.type_id_ping_response,
                    &messages::PingResponse {},
                )?;
            }
            Response::Invoke(seq, result) => {
                self.stream.serialize_with_type_id(
                    self.type_id_response,
                    &RpcResponse {
                        service_method: "Function.Invoke",
                        seq: seq,
                        error: None,
                    },
                )?;
                match result {
                    Ok(payload) => {
                        self.payload_buf.truncate(0);
                        ::serde_json::to_writer(&mut self.payload_buf, &payload)?;
                        self.stream.serialize_with_type_id(
                            self.type_id_invoke_response,
                            &messages::InvokeResponse {
                                payload: Bytes::new(&self.payload_buf),
                                error: None,
                            },
                        )?;
                    }
                    Err(err) => {
                        let invoke_error = messages::InvokeResponseError {
                            message: err.to_string(),
                            type_: "Error".to_owned(),
                            stack_trace: None,
                            should_exit: false,
                        };
                        self.stream.serialize_with_type_id(
                            self.type_id_invoke_response,
                            &messages::InvokeResponse {
                                payload: Bytes::new(&[]),
                                error: Some(invoke_error),
                            },
                        )?;
                    }
                }
            }
        }
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        loop {
            if self.flushing.remaining() == 0 {
                if self.stream.get_ref().len() == 0 {
                    return Ok(Async::Ready(()));
                } else {
                    // Reset flush buffer, then swap with stream buffer.
                    self.flushing.set_position(0);
                    self.flushing.get_mut().truncate(0);
                    ::std::mem::swap(self.stream.get_mut(), self.flushing.get_mut());
                }
            }
            try_ready!(self.write.write_buf(&mut self.flushing));
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::Sink;
    use tokio_io::io::AllowStdIo;

    use super::{Encoder, Response};

    #[test]
    fn encode_messages() {
        let mut buffer = Vec::<u8>::new();
        {
            let io = AllowStdIo::new(&mut buffer);
            let mut encoder = Encoder::<_, String>::new(io).unwrap();
            encoder.start_send(Response::Ping(0)).unwrap();
            encoder.poll_complete().unwrap();
            encoder
                .start_send(Response::Invoke(1, Ok("Hello ƛ!".to_owned())))
                .unwrap();
            encoder.poll_complete().unwrap();
        }

        assert_eq!(
            buffer,
            vec![
                58, 255, 129, 3, 1, 1, 8, 82, 101, 115, 112, 111, 110, 115, 101, 1, 255, 130, 0, 1,
                3, 1, 13, 83, 101, 114, 118, 105, 99, 101, 77, 101, 116, 104, 111, 100, 1, 12, 0,
                1, 3, 83, 101, 113, 1, 6, 0, 1, 5, 69, 114, 114, 111, 114, 1, 12, 0, 0, 0, 24, 255,
                131, 3, 1, 1, 12, 80, 105, 110, 103, 82, 101, 115, 112, 111, 110, 115, 101, 1, 255,
                132, 0, 0, 0, 73, 255, 133, 3, 1, 1, 31, 73, 110, 118, 111, 107, 101, 82, 101, 115,
                112, 111, 110, 115, 101, 95, 69, 114, 114, 111, 114, 95, 83, 116, 97, 99, 107, 70,
                114, 97, 109, 101, 1, 255, 134, 0, 1, 3, 1, 4, 80, 97, 116, 104, 1, 12, 0, 1, 4,
                76, 105, 110, 101, 1, 4, 0, 1, 5, 76, 97, 98, 101, 108, 1, 12, 0, 0, 0, 13, 255,
                135, 2, 1, 2, 255, 136, 0, 1, 255, 134, 0, 0, 86, 255, 137, 3, 1, 1, 20, 73, 110,
                118, 111, 107, 101, 82, 101, 115, 112, 111, 110, 115, 101, 95, 69, 114, 114, 111,
                114, 1, 255, 138, 0, 1, 4, 1, 7, 77, 101, 115, 115, 97, 103, 101, 1, 12, 0, 1, 4,
                84, 121, 112, 101, 1, 12, 0, 1, 10, 83, 116, 97, 99, 107, 84, 114, 97, 99, 101, 1,
                255, 136, 0, 1, 10, 83, 104, 111, 117, 108, 100, 69, 120, 105, 116, 1, 2, 0, 0, 0,
                51, 255, 139, 3, 1, 1, 14, 73, 110, 118, 111, 107, 101, 82, 101, 115, 112, 111,
                110, 115, 101, 1, 255, 140, 0, 1, 2, 1, 7, 80, 97, 121, 108, 111, 97, 100, 1, 10,
                0, 1, 5, 69, 114, 114, 111, 114, 1, 255, 138, 0, 0, 0, 18, 255, 130, 1, 13, 70,
                117, 110, 99, 116, 105, 111, 110, 46, 80, 105, 110, 103, 0, 3, 255, 132, 0, 22,
                255, 130, 1, 15, 70, 117, 110, 99, 116, 105, 111, 110, 46, 73, 110, 118, 111, 107,
                101, 1, 1, 0, 16, 255, 140, 1, 11, 34, 72, 101, 108, 108, 111, 32, 198, 155, 33,
                34, 0,
            ]
        )
    }
}
