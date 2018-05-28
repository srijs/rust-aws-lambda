use std::io::Read;
use std::marker::PhantomData;
use std::time::Duration;

use gob::StreamDeserializer;
use serde::de::DeserializeOwned;

use super::messages;
use context;

#[derive(Deserialize)]
struct RpcRequest<'a> {
    #[serde(rename = "ServiceMethod")]
    service_method: &'a str,
    #[serde(rename = "Seq", default)]
    seq: u64,
}

pub enum Request<T> {
    Ping(u64),
    Invoke(u64, Duration, ::context::LambdaContext, T),
}

pub struct Decoder<R, T> {
    stream: StreamDeserializer<R>,
    _phan: PhantomData<T>,
}

impl<R, T> Decoder<R, T> {
    pub fn new(r: R) -> Decoder<R, T> {
        Decoder {
            stream: StreamDeserializer::new(r),
            _phan: PhantomData,
        }
    }
}

impl<R, T> Iterator for Decoder<R, T>
where
    R: Read,
    T: DeserializeOwned,
{
    type Item = Request<T>;

    fn next(&mut self) -> Option<Request<T>> {
        let (seq, is_invoke) = {
            match self.stream.deserialize::<RpcRequest<'_>>().unwrap() {
                None => {
                    return None;
                }
                Some(req) => match req.service_method {
                    "Function.Ping" => (req.seq, false),
                    "Function.Invoke" => (req.seq, true),
                    _ => panic!("unknown service method"),
                },
            }
        };

        if !is_invoke {
            self.stream
                .deserialize::<messages::PingRequest>()
                .unwrap()
                .unwrap();
            return Some(Request::Ping(seq));
        }

        let message = self.stream
            .deserialize::<messages::InvokeRequest>()
            .unwrap()
            .unwrap();

        let identity = context::CognitoIdentity {
            cognito_identity_id: message.cognito_identity_id.map(|s| s.to_owned()),
            cognito_identity_pool_id: message.cognito_identity_pool_id.map(|s| s.to_owned()),
        };

        let ctx = context::LambdaContext {
            aws_request_id: message.request_id.to_owned(),
            invoked_function_arn: message.invoked_function_arn.to_owned(),
            identity: identity,
            client_context: None,
        };

        let deadline = Duration::new(message.deadline.secs as u64, message.deadline.nanos as u32);

        let payload = ::serde_json::from_slice(message.payload.as_ref()).unwrap();

        Some(Request::Invoke(seq, deadline, ctx, payload))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Decoder, Request};

    #[test]
    fn decode_messages() {
        let bytes = vec![
            47, 255, 129, 3, 1, 1, 7, 82, 101, 113, 117, 101, 115, 116, 1, 255, 130, 0, 1, 2, 1,
            13, 83, 101, 114, 118, 105, 99, 101, 77, 101, 116, 104, 111, 100, 1, 12, 0, 1, 3, 83,
            101, 113, 1, 6, 0, 0, 0, 18, 255, 130, 1, 13, 70, 117, 110, 99, 116, 105, 111, 110, 46,
            80, 105, 110, 103, 0, 23, 255, 131, 3, 1, 1, 11, 80, 105, 110, 103, 82, 101, 113, 117,
            101, 115, 116, 1, 255, 132, 0, 0, 0, 3, 255, 132, 0, 22, 255, 130, 1, 15, 70, 117, 110,
            99, 116, 105, 111, 110, 46, 73, 110, 118, 111, 107, 101, 1, 1, 0, 255, 173, 255, 133,
            3, 1, 1, 13, 73, 110, 118, 111, 107, 101, 82, 101, 113, 117, 101, 115, 116, 1, 255,
            134, 0, 1, 8, 1, 7, 80, 97, 121, 108, 111, 97, 100, 1, 10, 0, 1, 9, 82, 101, 113, 117,
            101, 115, 116, 73, 100, 1, 12, 0, 1, 12, 88, 65, 109, 122, 110, 84, 114, 97, 99, 101,
            73, 100, 1, 12, 0, 1, 8, 68, 101, 97, 100, 108, 105, 110, 101, 1, 255, 136, 0, 1, 18,
            73, 110, 118, 111, 107, 101, 100, 70, 117, 110, 99, 116, 105, 111, 110, 65, 114, 110,
            1, 12, 0, 1, 17, 67, 111, 103, 110, 105, 116, 111, 73, 100, 101, 110, 116, 105, 116,
            121, 73, 100, 1, 12, 0, 1, 21, 67, 111, 103, 110, 105, 116, 111, 73, 100, 101, 110,
            116, 105, 116, 121, 80, 111, 111, 108, 73, 100, 1, 12, 0, 1, 13, 67, 108, 105, 101,
            110, 116, 67, 111, 110, 116, 101, 120, 116, 1, 10, 0, 0, 0, 59, 255, 135, 3, 1, 1, 23,
            73, 110, 118, 111, 107, 101, 82, 101, 113, 117, 101, 115, 116, 95, 84, 105, 109, 101,
            115, 116, 97, 109, 112, 1, 255, 136, 0, 1, 2, 1, 7, 83, 101, 99, 111, 110, 100, 115, 1,
            4, 0, 1, 5, 78, 97, 110, 111, 115, 1, 4, 0, 0, 0, 255, 244, 255, 134, 1, 49, 123, 34,
            107, 101, 121, 51, 34, 58, 34, 118, 97, 108, 117, 101, 51, 34, 44, 34, 107, 101, 121,
            50, 34, 58, 34, 118, 97, 108, 117, 101, 50, 34, 44, 34, 107, 101, 121, 49, 34, 58, 34,
            118, 97, 108, 117, 101, 49, 34, 125, 1, 36, 50, 101, 100, 56, 48, 101, 52, 101, 45, 54,
            49, 57, 54, 45, 49, 49, 101, 56, 45, 56, 55, 54, 97, 45, 52, 102, 52, 49, 98, 100, 56,
            57, 51, 99, 52, 50, 1, 74, 82, 111, 111, 116, 61, 49, 45, 53, 98, 48, 97, 56, 52, 49,
            53, 45, 49, 102, 98, 99, 49, 52, 50, 55, 98, 98, 56, 54, 56, 50, 53, 49, 54, 51, 48,
            50, 97, 53, 53, 101, 59, 80, 97, 114, 101, 110, 116, 61, 49, 48, 101, 97, 99, 54, 101,
            99, 52, 50, 50, 48, 54, 99, 53, 48, 59, 83, 97, 109, 112, 108, 101, 100, 61, 48, 1, 1,
            252, 182, 21, 8, 50, 1, 252, 3, 234, 124, 228, 0, 1, 60, 97, 114, 110, 58, 97, 119,
            115, 58, 108, 97, 109, 98, 100, 97, 58, 97, 112, 45, 115, 111, 117, 116, 104, 101, 97,
            115, 116, 45, 50, 58, 55, 55, 49, 51, 49, 54, 48, 52, 51, 48, 51, 57, 58, 102, 117,
            110, 99, 116, 105, 111, 110, 58, 116, 101, 115, 116, 70, 110, 71, 111, 0,
        ];

        let mut decoder = Decoder::<_, HashMap<String, String>>::new(::std::io::Cursor::new(bytes));

        let request1 = decoder.next().unwrap();
        match request1 {
            Request::Ping(seq) => {
                assert_eq!(0, seq);
            }
            _ => panic!("wrong request type"),
        }

        let request2 = decoder.next().unwrap();
        match request2 {
            Request::Invoke(seq, deadline, ctx, payload) => {
                assert_eq!(1, seq);
                assert_eq!("2ed80e4e-6196-11e8-876a-4f41bd893c42", ctx.aws_request_id);
                assert_eq!(1527415833, deadline.as_secs());
                assert_eq!(32849522, deadline.subsec_nanos());
                assert_eq!(3, payload.len());
                assert_eq!("value1", payload["key1"]);
                assert_eq!("value2", payload["key2"]);
                assert_eq!("value3", payload["key3"]);
            }
            _ => panic!("wrong request type"),
        }

        assert!(decoder.next().is_none());
    }
}
