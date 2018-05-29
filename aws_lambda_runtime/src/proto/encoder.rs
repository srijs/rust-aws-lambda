use std::io::Write;
use std::marker::PhantomData;

use failure::Error;
use gob::StreamSerializer;
use serde::Serialize;
use serde_bytes::ByteBuf;

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

pub struct Encoder<W, T> {
    stream: StreamSerializer<W>,
    _phan: PhantomData<T>,
}

impl<W, T> Encoder<W, T> {
    pub fn new(w: W) -> Encoder<W, T> {
        Encoder {
            stream: StreamSerializer::new(w),
            _phan: PhantomData,
        }
    }

    pub fn encode(&mut self, res: Response<T>)
    where
        W: Write,
        T: Serialize,
    {
        match res {
            Response::Ping(seq) => {
                self.stream
                    .serialize(&RpcResponse {
                        service_method: "Function.Ping",
                        seq: seq,
                        error: None,
                    })
                    .unwrap();
                self.stream.serialize(&messages::PingResponse {}).unwrap()
            }
            Response::Invoke(seq, result) => {
                self.stream
                    .serialize(&RpcResponse {
                        service_method: "Function.Invoke",
                        seq: seq,
                        error: None,
                    })
                    .unwrap();
                match result {
                    Ok(payload) => {
                        let payload_bytes = ::serde_json::to_vec(&payload).unwrap();
                        self.stream.serialize(&messages::InvokeResponse {
                            payload: ByteBuf::from(payload_bytes),
                            error: None,
                        })
                    }
                    Err(err) => {
                        let invoke_error = messages::InvokeResponseError {
                            message: err.to_string(),
                            type_: "Error".to_owned(),
                            stack_trace: None,
                            should_exit: false,
                        };
                        self.stream.serialize(&messages::InvokeResponse {
                            payload: ByteBuf::new(),
                            error: Some(invoke_error),
                        })
                    }
                }.unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Encoder, Response};

    #[test]
    fn encode_messages() {
        let mut buffer = Vec::<u8>::new();
        {
            let mut encoder = Encoder::<_, String>::new(&mut buffer);
            encoder.encode(Response::Ping(0));
            encoder.encode(Response::Invoke(1, Ok("Hello ƛ!".to_owned())));
        }

        assert_eq!(
            buffer,
            vec![
                58, 255, 129, 3, 1, 1, 8, 82, 101, 115, 112, 111, 110, 115, 101, 1, 255, 130, 0, 1,
                3, 1, 13, 83, 101, 114, 118, 105, 99, 101, 77, 101, 116, 104, 111, 100, 1, 12, 0,
                1, 3, 83, 101, 113, 1, 6, 0, 1, 5, 69, 114, 114, 111, 114, 1, 12, 0, 0, 0, 18, 255,
                130, 1, 13, 70, 117, 110, 99, 116, 105, 111, 110, 46, 80, 105, 110, 103, 0, 24,
                255, 131, 3, 1, 1, 12, 80, 105, 110, 103, 82, 101, 115, 112, 111, 110, 115, 101, 1,
                255, 132, 0, 0, 0, 3, 255, 132, 0, 22, 255, 130, 1, 15, 70, 117, 110, 99, 116, 105,
                111, 110, 46, 73, 110, 118, 111, 107, 101, 1, 1, 0, 73, 255, 133, 3, 1, 1, 31, 73,
                110, 118, 111, 107, 101, 82, 101, 115, 112, 111, 110, 115, 101, 95, 69, 114, 114,
                111, 114, 95, 83, 116, 97, 99, 107, 70, 114, 97, 109, 101, 1, 255, 134, 0, 1, 3, 1,
                4, 80, 97, 116, 104, 1, 12, 0, 1, 4, 76, 105, 110, 101, 1, 4, 0, 1, 5, 76, 97, 98,
                101, 108, 1, 12, 0, 0, 0, 13, 255, 135, 2, 1, 2, 255, 136, 0, 1, 255, 134, 0, 0,
                86, 255, 137, 3, 1, 1, 20, 73, 110, 118, 111, 107, 101, 82, 101, 115, 112, 111,
                110, 115, 101, 95, 69, 114, 114, 111, 114, 1, 255, 138, 0, 1, 4, 1, 7, 77, 101,
                115, 115, 97, 103, 101, 1, 12, 0, 1, 4, 84, 121, 112, 101, 1, 12, 0, 1, 10, 83,
                116, 97, 99, 107, 84, 114, 97, 99, 101, 1, 255, 136, 0, 1, 10, 83, 104, 111, 117,
                108, 100, 69, 120, 105, 116, 1, 2, 0, 0, 0, 51, 255, 139, 3, 1, 1, 14, 73, 110,
                118, 111, 107, 101, 82, 101, 115, 112, 111, 110, 115, 101, 1, 255, 140, 0, 1, 2, 1,
                7, 80, 97, 121, 108, 111, 97, 100, 1, 10, 0, 1, 5, 69, 114, 114, 111, 114, 1, 255,
                138, 0, 0, 0, 16, 255, 140, 1, 11, 34, 72, 101, 108, 108, 111, 32, 198, 155, 33,
                34, 0,
            ]
        )
    }
}
