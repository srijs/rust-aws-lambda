use std::borrow::Cow;
use std::fmt;

use base64;
use http;
use serde::{de::Error as DeError, de::MapAccess, de::Visitor, Deserialize, Deserializer};

use body::Body;

#[derive(Debug)]
pub struct ApiGatewayProxyRequest(pub(crate) http::Request<Body>);

impl<'de> Deserialize<'de> for ApiGatewayProxyRequest {
    fn deserialize<D>(deserializer: D) -> Result<ApiGatewayProxyRequest, D::Error>
    where
        D: Deserializer<'de>,
    {
        ApiGatewayProxyRequestDef::deserialize(deserializer)
            .and_then(|def| def.try_into_http_request())
            .map(ApiGatewayProxyRequest)
    }
}

#[derive(Deserialize)]
struct ApiGatewayProxyRequestDef<'a> {
    #[serde(default, borrow)]
    path: Option<Cow<'a, str>>,
    #[serde(default, borrow, rename = "httpMethod")]
    http_method: Option<Cow<'a, str>>,
    #[serde(default)]
    headers: Option<DeserializeHeaders>,
    #[serde(default, borrow)]
    body: Option<Cow<'a, str>>,
    #[serde(default, rename = "isBase64Encoded")]
    is_base64_encoded: Option<bool>,
}

impl<'a> ApiGatewayProxyRequestDef<'a> {
    fn try_into_http_request<E: DeError>(self) -> Result<http::Request<Body>, E> {
        let mut builder = http::Request::builder();

        if let Some(path) = self.path {
            builder.uri(path.as_ref());
        }

        if let Some(http_method) = self.http_method {
            builder.method(http_method.as_ref());
        }

        let mut body = Body::default();
        if let Some(raw_body) = self.body {
            if self.is_base64_encoded.unwrap_or(false) {
                body = Body::from(
                    base64::decode(raw_body.as_ref().as_bytes()).map_err(|err| E::custom(err))?,
                );
            } else {
                body = Body::from(raw_body.into_owned());
            }
        }

        let mut req = builder.body(body).map_err(|err| E::custom(err))?;

        if let Some(DeserializeHeaders(headers)) = self.headers {
            ::std::mem::replace(req.headers_mut(), headers);
        }

        Ok(req)
    }
}

struct DeserializeHeaders(http::HeaderMap<http::header::HeaderValue>);

impl<'de> Deserialize<'de> for DeserializeHeaders {
    fn deserialize<D>(deserializer: D) -> Result<DeserializeHeaders, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor;

        impl<'de> Visitor<'de> for MapVisitor {
            type Value = DeserializeHeaders;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a header map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut headers = http::HeaderMap::new();
                while let Some((map_key, map_value)) = map.next_entry::<Cow<str>, Cow<str>>()? {
                    let header_name = map_key
                        .parse::<http::header::HeaderName>()
                        .map_err(|err| A::Error::custom(err))?;
                    let header_value = http::header::HeaderValue::from_shared(
                        map_value.into_owned().into(),
                    ).map_err(|err| A::Error::custom(err))?;
                    headers.append(header_name, header_value);
                }
                Ok(DeserializeHeaders(headers))
            }
        }

        deserializer.deserialize_map(MapVisitor)
    }
}

#[test]
fn deserialize_simple() {
    let input = include_str!("../tests/fixtures/request_simple.json");
    let ApiGatewayProxyRequest(req) =
        ::serde_json::from_str::<ApiGatewayProxyRequest>(&input).unwrap();

    assert_eq!(req.method(), http::Method::GET);
    assert_eq!(req.uri().path(), "/");
    assert_eq!(req.body().as_str().unwrap(), "");
}

#[test]
fn deserialize_complex() {
    let input = include_str!("../tests/fixtures/request_complex.json");
    let ApiGatewayProxyRequest(req) =
        ::serde_json::from_str::<ApiGatewayProxyRequest>(&input).unwrap();

    assert_eq!(req.method(), http::Method::POST);
    assert_eq!(req.uri().path(), "/path/to/resource");
    assert_eq!(req.body().as_str().unwrap(), "{\"test\":\"body\"}");
    assert_eq!(
        req.headers()["Accept"],
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
    );
    assert_eq!(
        req.headers()["Host"],
        "1234567890.execute-api.ap-southeast-2.amazonaws.com"
    );
}
