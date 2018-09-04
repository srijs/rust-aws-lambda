use serde::Deserialize;

#[derive(Debug, Clone)]
pub(crate) enum Inner {
    Empty,
    Utf8(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Body(pub(crate) Inner);

impl Body {
    pub fn as_bytes(&self) -> &[u8] {
        match self.0 {
            Inner::Empty => &[],
            Inner::Utf8(ref text) => text.as_bytes(),
            Inner::Binary(ref bytes) => bytes.as_slice(),
        }
    }

    pub fn as_str(&self) -> Result<&str, ::std::str::Utf8Error> {
        match self.0 {
            Inner::Empty => Ok(""),
            Inner::Utf8(ref text) => Ok(text.as_str()),
            Inner::Binary(ref bytes) => ::std::str::from_utf8(bytes),
        }
    }

    pub fn decode_json<'a, T>(&'a self) -> Result<T, ::serde_json::Error>
    where
        T: Deserialize<'a>,
    {
        ::serde_json::from_slice(self.as_bytes())
    }
}

impl AsRef<[u8]> for Body {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Default for Body {
    fn default() -> Body {
        Body(Inner::Empty)
    }
}

impl From<String> for Body {
    fn from(text: String) -> Body {
        Body(Inner::Utf8(text))
    }
}

impl<'a> From<&'a str> for Body {
    fn from(text: &'a str) -> Body {
        Body(Inner::Utf8(text.to_owned()))
    }
}

impl From<Vec<u8>> for Body {
    fn from(bytes: Vec<u8>) -> Body {
        Body(Inner::Binary(bytes))
    }
}

impl<'a> From<&'a [u8]> for Body {
    fn from(bytes: &'a [u8]) -> Body {
        Body(Inner::Binary(bytes.to_vec()))
    }
}
