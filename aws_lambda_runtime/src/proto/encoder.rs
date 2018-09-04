use std::fmt::Write;
use std::marker::PhantomData;

use backtrace_parser::Backtrace;
use bytes::Buf;
use failure::Error;
use futures::{AsyncSink, Poll, Sink, StartSend};
use gob::{ser::OutputBuffer, ser::TypeId, StreamSerializer};
use serde::Serialize;
use serde_bytes::Bytes;
use serde_schema::SchemaSerialize;
use tokio::io::AsyncWrite;

use super::super::error::ConnectionError;
use super::messages;

#[derive(Serialize, SchemaSerialize)]
#[cfg_attr(test, derive(Deserialize))]
#[serde(rename = "Response")]
struct RpcResponse<'a> {
    #[serde(rename = "ServiceMethod")]
    service_method: &'a str,
    #[serde(rename = "Seq")]
    seq: u64,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

pub enum Response<T> {
    Ping(u64),
    Invoke(u64, Result<T, Error>),
}

pub(crate) struct Encoder<W, T>
where
    W: AsyncWrite,
{
    write: W,
    payload_buf: Vec<u8>,
    error_encoder: InvokeResponseErrorEncoder,
    stream: StreamSerializer<OutputBuffer>,
    _phan: PhantomData<T>,
    type_id_response: TypeId,
    type_id_ping_response: TypeId,
    type_id_invoke_response: TypeId,
}

impl<W, T> Encoder<W, T>
where
    W: AsyncWrite,
{
    pub fn new(w: W) -> Encoder<W, T> {
        let payload_buf = Vec::with_capacity(4096);
        let error_encoder = InvokeResponseErrorEncoder::default();

        let mut stream = StreamSerializer::new_with_buffer();
        let type_id_response = RpcResponse::schema_register(stream.schema_mut()).unwrap();
        let type_id_ping_response =
            messages::PingResponse::schema_register(stream.schema_mut()).unwrap();
        let type_id_invoke_response =
            messages::InvokeResponse::schema_register(stream.schema_mut()).unwrap();

        Encoder {
            write: w,
            payload_buf,
            error_encoder,
            stream,
            type_id_response,
            type_id_ping_response,
            type_id_invoke_response,
            _phan: PhantomData,
        }
    }

    fn encode_ping(&mut self, seq: u64) -> Result<(), ConnectionError> {
        self.stream.serialize_with_type_id(
            self.type_id_response,
            &RpcResponse {
                service_method: messages::SERVICE_METHOD_PING,
                seq,
                error: None,
            },
        )?;
        self.stream
            .serialize_with_type_id(self.type_id_ping_response, &messages::PingResponse {})?;
        Ok(())
    }

    fn encode_invoke(&mut self, seq: u64, result: Result<T, Error>) -> Result<(), ConnectionError>
    where
        T: Serialize,
    {
        self.stream.serialize_with_type_id(
            self.type_id_response,
            &RpcResponse {
                service_method: messages::SERVICE_METHOD_INVOKE,
                seq,
                error: None,
            },
        )?;
        match result {
            Ok(payload) => {
                self.payload_buf.clear();
                match ::serde_json::to_writer(&mut self.payload_buf, &payload) {
                    Ok(()) => {
                        self.stream.serialize_with_type_id(
                            self.type_id_invoke_response,
                            &messages::InvokeResponse::Payload(Bytes::new(&self.payload_buf)),
                        )?;
                    }
                    Err(err) => {
                        // We failed to encode the invoke response payload. Instead
                        // of bubbling it up as a connection error, encode the error
                        // as part of the response.
                        let invoke_error = self.error_encoder.encode_serialize_error(&err);
                        self.stream.serialize_with_type_id(
                            self.type_id_invoke_response,
                            &messages::InvokeResponse::Error(invoke_error),
                        )?;
                    }
                }
            }
            Err(err) => {
                let invoke_error = self.error_encoder.encode(&err);
                self.stream.serialize_with_type_id(
                    self.type_id_invoke_response,
                    &messages::InvokeResponse::Error(invoke_error),
                )?;
            }
        }
        Ok(())
    }
}

impl<W, T> Sink for Encoder<W, T>
where
    W: AsyncWrite,
    T: Serialize,
{
    type SinkItem = Response<T>;
    type SinkError = ConnectionError;

    fn start_send(&mut self, res: Response<T>) -> StartSend<Self::SinkItem, Self::SinkError> {
        match res {
            Response::Ping(seq) => self.encode_ping(seq)?,
            Response::Invoke(seq, result) => self.encode_invoke(seq, result)?,
        }
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        loop {
            if !self.stream.get_ref().has_remaining() {
                return Ok(self.write.poll_flush()?);
            }
            try_ready!(self.write.write_buf(self.stream.get_mut()));
        }
    }
}

#[derive(Default)]
struct InvokeResponseErrorEncoder {
    message_buf: String,
    backtrace_buf: String,
}

impl InvokeResponseErrorEncoder {
    fn encode<'a>(&'a mut self, err: &Error) -> messages::InvokeResponseError<'a> {
        // Attempt to extract a stack trace from the error,
        // by rendering the opaque backtrace into a buffer,
        // and then running a parser over it.
        //
        // If the parser fails, no stack trace will be included.
        self.backtrace_buf.clear();
        write!(self.backtrace_buf, "{}", err.backtrace()).unwrap();
        let stack_trace = Backtrace::parse(&self.backtrace_buf)
            .map(messages::InvokeResponseErrorStackTrace)
            .ok();
        // Render the rest of the error.
        self.message_buf.clear();
        write!(self.message_buf, "{}", err).unwrap();
        messages::InvokeResponseError {
            message: &self.message_buf,
            type_: "Error",
            stack_trace,
            should_exit: false,
        }
    }

    fn encode_serialize_error<'a>(
        &'a mut self,
        err: &::serde_json::Error,
    ) -> messages::InvokeResponseError<'a> {
        self.message_buf.clear();
        write!(self.message_buf, "failed to serialize response: {}", err).unwrap();
        messages::InvokeResponseError {
            message: &self.message_buf,
            type_: "Error",
            stack_trace: None,
            should_exit: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::Sink;
    use gob::StreamDeserializer;
    use partial_io::{GenWouldBlock, PartialAsyncWrite, PartialWithErrors};

    use super::*;

    #[test]
    fn serialization_error() {
        struct FailToSerialize;
        impl ::serde::Serialize for FailToSerialize {
            fn serialize<S: ::serde::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(::serde::ser::Error::custom("foo bar reason"))
            }
        }

        let mut buffer = ::std::io::Cursor::new(Vec::<u8>::new());
        {
            let mut encoder = Encoder::<_, FailToSerialize>::new(&mut buffer).wait();
            encoder
                .send(Response::Invoke(1, Ok(FailToSerialize)))
                .unwrap();
            encoder.flush().unwrap();
        };
        buffer.set_position(0);

        let mut de = StreamDeserializer::new(buffer);

        {
            let header = de.deserialize::<RpcResponse>().unwrap().unwrap();

            assert_eq!(header.service_method, "Function.Invoke");
            assert_eq!(header.seq, 1);
            assert_eq!(header.error, None);
        }

        {
            let body = de.deserialize::<messages::InvokeResponse>()
                .unwrap()
                .unwrap();

            if let messages::InvokeResponse::Error(err) = body {
                assert_eq!(err.type_, "Error");
                assert_eq!(err.message, "failed to serialize response: foo bar reason");
                assert_eq!(err.should_exit, false);
            } else {
                panic!("not an invoke error")
            }
        }
    }

    quickcheck! {
        fn encode_messages(seq: PartialWithErrors<GenWouldBlock>) -> bool {
            let mut write = ::std::io::Cursor::new(Vec::<u8>::new());
            {
                let pwrite = PartialAsyncWrite::new(&mut write, seq);
                let mut encoder = Encoder::<_, String>::new(pwrite).wait();
                encoder.send(Response::Ping(0)).unwrap();
                encoder.flush().unwrap();
                encoder
                    .send(Response::Invoke(1, Ok("Hello ƛ!".to_owned())))
                    .unwrap();
                encoder.flush().unwrap();
            }

            write.into_inner() == vec![
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
        }
    }
}
