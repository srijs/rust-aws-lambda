use serde::{Deserializer, de::IgnoredAny, de::Visitor};
use serde_json::{Deserializer as JsonDeserializer, Error, de::SliceRead};

pub struct PayloadDeserializer<'a> {
    inner: JsonDeserializer<SliceRead<'a>>,
}

impl<'a> PayloadDeserializer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        PayloadDeserializer {
            inner: JsonDeserializer::from_slice(input),
        }
    }
}

impl<'de> Deserializer<'de> for PayloadDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_any(visitor)
    }

    #[inline]
    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_bool(visitor)
    }

    #[inline]
    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_i8(visitor)
    }

    #[inline]
    fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_i16(visitor)
    }

    #[inline]
    fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_i32(visitor)
    }

    #[inline]
    fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_i64(visitor)
    }

    #[inline]
    fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_u8(visitor)
    }

    #[inline]
    fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_u16(visitor)
    }

    #[inline]
    fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_u32(visitor)
    }

    #[inline]
    fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_u64(visitor)
    }

    #[inline]
    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_f32(visitor)
    }

    #[inline]
    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_f64(visitor)
    }

    #[inline]
    fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_char(visitor)
    }

    #[inline]
    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_string(visitor)
    }

    #[inline]
    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_bytes(visitor)
    }

    #[inline]
    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_byte_buf(visitor)
    }

    #[inline]
    fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_option(visitor)
    }

    #[inline]
    fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_ignored_any(IgnoredAny)?;
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_unit_struct<V>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_unit_struct(name, visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        mut self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_newtype_struct(name, visitor)
    }

    #[inline]
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_tuple(len, visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        mut self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_tuple_struct(name, len, visitor)
    }

    #[inline]
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_map(visitor)
    }

    #[inline]
    fn deserialize_struct<V>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_struct(name, fields, visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_enum(name, variants, visitor)
    }

    #[inline]
    fn deserialize_identifier<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_identifier(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner.deserialize_ignored_any(visitor)
    }
}

#[test]
fn deserializes_unit_from_any() {
    use serde::Deserialize;

    let () = <()>::deserialize(PayloadDeserializer::new("{}".as_bytes())).unwrap();
    let () = <()>::deserialize(PayloadDeserializer::new("[]".as_bytes())).unwrap();
    let () = <()>::deserialize(PayloadDeserializer::new("123".as_bytes())).unwrap();
    let () = <()>::deserialize(PayloadDeserializer::new("\"foo\"".as_bytes())).unwrap();
    let () = <()>::deserialize(PayloadDeserializer::new("null".as_bytes())).unwrap();
}
