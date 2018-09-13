use base64::display::Base64Display;
use http;
use serde::{ser::Error as SerError, ser::SerializeMap, Serialize, Serializer};

use body::{self, Body};

#[derive(Debug)]
pub struct ApiGatewayProxyResponse(pub(crate) http::Response<Body>);

impl Default for ApiGatewayProxyResponse {
    fn default() -> Self {
        ApiGatewayProxyResponse(http::Response::default())
    }
}

impl Serialize for ApiGatewayProxyResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ApiGatewayProxyResponseDef::from_http_response(&self.0).serialize(serializer)
    }
}

#[derive(Serialize)]
struct ApiGatewayProxyResponseDef<'a> {
    #[serde(rename = "statusCode")]
    status_code: i64,
    headers: SerializeHeaders<'a>,
    body: Option<SerializeBody<'a>>,
    #[serde(rename = "isBase64Encoded")]
    is_base64_encoded: bool,
}

impl<'a> ApiGatewayProxyResponseDef<'a> {
    fn from_http_response(http_res: &'a http::Response<Body>) -> Self {
        let (body, is_base64_encoded) = match http_res.body().0 {
            body::Inner::Empty => (None, false),
            body::Inner::Utf8(ref text) => (Some(SerializeBody::Utf8(text)), false),
            body::Inner::Binary(ref bytes) => (Some(SerializeBody::Binary(bytes)), true),
        };

        ApiGatewayProxyResponseDef {
            status_code: http_res.status().as_u16() as i64,
            headers: SerializeHeaders(http_res.headers()),
            body,
            is_base64_encoded,
        }
    }
}

enum SerializeBody<'a> {
    Utf8(&'a str),
    Binary(&'a [u8]),
}

impl<'a> Serialize for SerializeBody<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SerializeBody::Utf8(data) => serializer.serialize_str(data),
            SerializeBody::Binary(data) => serializer.collect_str(&Base64Display::standard(data)),
        }
    }
}

struct SerializeHeaders<'a>(&'a http::HeaderMap<http::header::HeaderValue>);

impl<'a> Serialize for SerializeHeaders<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.keys_len()))?;
        for key in self.0.keys() {
            let map_value = self.0[key].to_str().map_err(|err| S::Error::custom(err))?;
            map.serialize_entry(key.as_str(), map_value)?;
        }
        map.end()
    }
}

#[test]
fn serialize_simple() {
    let res = http::Response::builder()
        .status(200)
        .body(Default::default())
        .unwrap();

    let json = ::serde_json::to_string(&ApiGatewayProxyResponse(res)).unwrap();

    assert_eq!(
        json,
        "{\"statusCode\":200,\"headers\":{},\"body\":null,\"isBase64Encoded\":false}"
    );
}

#[test]
fn serialize_with_utf8_body() {
    let res = http::Response::builder()
        .status(200)
        .body("Hello World!".into())
        .unwrap();

    let json = ::serde_json::to_string(&ApiGatewayProxyResponse(res)).unwrap();

    assert_eq!(
        json,
        "{\"statusCode\":200,\"headers\":{},\"body\":\"Hello World!\",\"isBase64Encoded\":false}"
    );
}

#[test]
fn serialize_default() {
    let json = ::serde_json::to_string(&ApiGatewayProxyResponse::default()).unwrap();
    assert_eq!(
        json,
        "{\"statusCode\":200,\"headers\":{},\"body\":null,\"isBase64Encoded\":false}"
    );
}
