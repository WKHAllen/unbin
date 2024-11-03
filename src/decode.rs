//! Decoding implementation.

use std::marker::PhantomData;

use crate::read::Read;
use crate::util::*;
use crate::{Error, ValueType};
use serde::de::{
    DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor,
};
use serde::Deserializer;

/// The binary decoder.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decoder<'de, R>(R, PhantomData<&'de ()>)
where
    R: Read<'de>;

impl<'de, R> Decoder<'de, R>
where
    R: Read<'de>,
{
    /// Constructs a new binary decoder.
    pub fn new(reader: R) -> Self {
        Self(reader, PhantomData)
    }

    /// Returns a mutable reference to the underlying reader.
    pub fn reader(&mut self) -> &mut R {
        &mut self.0
    }

    /// Unwraps and returns the underlying reader.
    pub fn into_reader(self) -> R {
        self.0
    }
}

impl<'de, 'a, R> Deserializer<'de> for &'a mut Decoder<'de, R>
where
    R: Read<'de>,
{
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::CannotDeserializeAny)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<1>()?;
        let value = match bytes[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidBytes {
                ty: ValueType::Bool,
                bytes: bytes.to_vec(),
            }),
        }?;
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<1>()?;
        visitor.visit_i8(i8::from_be_bytes(bytes))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<2>()?;
        visitor.visit_i16(i16::from_be_bytes(bytes))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<4>()?;
        visitor.visit_i32(i32::from_be_bytes(bytes))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<8>()?;
        visitor.visit_i64(i64::from_be_bytes(bytes))
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<16>()?;
        visitor.visit_i128(i128::from_be_bytes(bytes))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<1>()?;
        visitor.visit_u8(bytes[0])
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<2>()?;
        visitor.visit_u16(u16::from_be_bytes(bytes))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<4>()?;
        visitor.visit_u32(u32::from_be_bytes(bytes))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<8>()?;
        visitor.visit_u64(u64::from_be_bytes(bytes))
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<16>()?;
        visitor.visit_u128(u128::from_be_bytes(bytes))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<4>()?;
        visitor.visit_f32(f32::from_be_bytes(bytes))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_n_array::<8>()?;
        visitor.visit_f64(f64::from_be_bytes(bytes))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len = self.0.read_n_array::<1>()?;
        let decoded_len = decode_len_small(len[0]);
        let mut bytes = [0; 4];
        self.0.read_exact(&mut bytes[4 - decoded_len..])?;
        let chr = std::str::from_utf8(&bytes[4 - decoded_len..])?
            .chars()
            .take(1)
            .next()
            .unwrap();
        visitor.visit_char(chr)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.visit_str(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_bytes_with_large_len()?;
        let string = std::str::from_utf8(&bytes)?;
        visitor.visit_string(string.to_owned())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.visit_bytes(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let bytes = self.0.read_bytes_with_large_len()?;
        visitor.visit_byte_buf(bytes)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let discriminant = self.0.read_n_array::<1>()?;

        match discriminant[0] {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
            _ => Err(Error::InvalidBytes {
                ty: ValueType::Option,
                bytes: discriminant.to_vec(),
            }),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len1 = self.0.read_n_array::<1>()?;
        let decoded_len1 = decode_len_small(len1[0]);
        let len2 = self.0.read_n_vec(decoded_len1)?;
        let decoded_len2 = decode_len_large(&len2);
        visitor.visit_seq(SeqDecoder::new(self, decoded_len2))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqDecoder::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqDecoder::new(self, len))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let len1 = self.0.read_n_array::<1>()?;
        let decoded_len1 = decode_len_small(len1[0]);
        let len2 = self.0.read_n_vec(decoded_len1)?;
        let decoded_len2 = decode_len_large(&len2);
        visitor.visit_map(MapDecoder::new(self, decoded_len2))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqDecoder::new(self, fields.len()))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(EnumDecoder::new(self))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::CannotDeserializeIdentifier)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::CannotDeserializeAny)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

/// Decodes a sequence.
pub struct SeqDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    /// The underlying decoder.
    decoder: &'a mut Decoder<'de, R>,
    /// The number of items in the sequence.
    len: usize,
}

impl<'de, 'a, R> SeqDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    /// Creates a new sequence decoder.
    pub fn new(decoder: &'a mut Decoder<'de, R>, len: usize) -> Self {
        Self { decoder, len }
    }
}

impl<'de, 'a, R> SeqAccess<'de> for SeqDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            let value = seed.deserialize(&mut *self.decoder)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

/// Decodes a map.
pub struct MapDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    /// The underlying decoder.
    decoder: &'a mut Decoder<'de, R>,
    /// The number of items in the map.
    len: usize,
}

impl<'de, 'a, R> MapDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    /// Creates a new map decoder.
    pub fn new(decoder: &'a mut Decoder<'de, R>, len: usize) -> Self {
        Self { decoder, len }
    }
}

impl<'de, 'a, R> MapAccess<'de> for MapDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            let key = seed.deserialize(&mut *self.decoder)?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.decoder)?;
        Ok(value)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

/// Decodes an enum.
pub struct EnumDecoder<'de, 'a, R>(&'a mut Decoder<'de, R>)
where
    R: Read<'de>;

impl<'de, 'a, R> EnumDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    /// Creates a new enum decoder.
    pub fn new(decoder: &'a mut Decoder<'de, R>) -> Self {
        Self(decoder)
    }
}

impl<'de, 'a, R> EnumAccess<'de> for EnumDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    type Error = Error;
    type Variant = VariantDecoder<'de, 'a, R>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant_index = self.0 .0.read_n_array::<1>()?[0];
        let value: crate::Result<_> = seed.deserialize(variant_index.into_deserializer());
        Ok((value?, VariantDecoder::new(self.0)))
    }
}

/// Decodes an enum variant.
pub struct VariantDecoder<'de, 'a, R>(&'a mut Decoder<'de, R>)
where
    R: Read<'de>;

impl<'de, 'a, R> VariantDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    /// Creates a new enum variant decoder.
    pub fn new(decoder: &'a mut Decoder<'de, R>) -> Self {
        Self(decoder)
    }
}

impl<'de, 'a, R> VariantAccess<'de> for VariantDecoder<'de, 'a, R>
where
    R: Read<'de>,
{
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.0)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_struct("", fields, visitor)
    }
}
