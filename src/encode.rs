//! Encoding implementation.

use crate::util::*;
use crate::write::Write;
use crate::Error;
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::Serializer;

/// The binary encoder.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Encoder<W>(W)
where
    W: Write;

impl<W> Encoder<W>
where
    W: Write,
{
    /// Constructs a new binary encoder.
    pub fn new(writer: W) -> Self {
        Self(writer)
    }

    /// Returns a mutable reference to the underlying writer.
    pub fn writer(&mut self) -> &mut W {
        &mut self.0
    }

    /// Unwraps and returns the underlying writer.
    pub fn into_writer(self) -> W {
        self.0
    }
}

impl<'a, W> Serializer for &'a mut Encoder<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SeqEncoder<'a, W>;
    type SerializeTuple = TupleEncoder<'a, W>;
    type SerializeTupleStruct = TupleStructEncoder<'a, W>;
    type SerializeTupleVariant = TupleVariantEncoder<'a, W>;
    type SerializeMap = MapEncoder<'a, W>;
    type SerializeStruct = StructEncoder<'a, W>;
    type SerializeStructVariant = StructVariantEncoder<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&[v as u8])?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&[v])?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let len = v.len_utf8();
        let encoded_len = encode_len_small(len);
        let mut bytes = [encoded_len; 5];
        v.encode_utf8(&mut bytes[1..]);
        self.0.write_all(&bytes[..len + 1])?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let mut bytes = encode_len_large(v.len());
        bytes.extend_from_slice(v.as_bytes());
        self.0.write_all(&bytes)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut bytes = encode_len_large(v.len());
        bytes.extend_from_slice(v);
        self.0.write_all(&bytes)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.0.write_all(&[0])?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.0.write_all(&[1])?;
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        if variant_index < 256 {
            self.0.write_all(&(variant_index as u8).to_be_bytes())?;
            Ok(())
        } else {
            Err(Error::TooManyVariants(name))
        }
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if variant_index < 256 {
            self.0.write_all(&(variant_index as u8).to_be_bytes())?;
            value.serialize(self)?;
            Ok(())
        } else {
            Err(Error::TooManyVariants(name))
        }
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => SeqEncoder::new(self, len),
            None => Err(Error::UnknownSeqLengthNotAllowed),
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(TupleEncoder::new(self))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(TupleStructEncoder::new(self))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        TupleVariantEncoder::new(self, name, variant_index)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            Some(len) => MapEncoder::new(self, len),
            None => Err(Error::UnknownMapLengthNotAllowed),
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructEncoder::new(self))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        StructVariantEncoder::new(self, name, variant_index)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

/// Encodes a sequence to binary.
pub struct SeqEncoder<'a, W>(&'a mut Encoder<W>)
where
    W: Write;

impl<'a, W> SeqEncoder<'a, W>
where
    W: Write,
{
    /// Creates a new sequence encoder.
    pub fn new(encoder: &'a mut Encoder<W>, len: usize) -> crate::Result<Self> {
        let encoded_len = encode_len_large(len);
        encoder.0.write_all(&encoded_len)?;
        Ok(Self(encoder))
    }
}

impl<'a, W> SerializeSeq for SeqEncoder<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.0)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Encodes a tuple to binary.
pub struct TupleEncoder<'a, W>(&'a mut Encoder<W>)
where
    W: Write;

impl<'a, W> TupleEncoder<'a, W>
where
    W: Write,
{
    /// Creates a new tuple encoder.
    pub fn new(encoder: &'a mut Encoder<W>) -> Self {
        Self(encoder)
    }
}

impl<'a, W> SerializeTuple for TupleEncoder<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.0)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Encodes a tuple struct to binary.
pub struct TupleStructEncoder<'a, W>(&'a mut Encoder<W>)
where
    W: Write;

impl<'a, W> TupleStructEncoder<'a, W>
where
    W: Write,
{
    /// Creates a new tuple struct encoder.
    pub fn new(encoder: &'a mut Encoder<W>) -> Self {
        Self(encoder)
    }
}

impl<'a, W> SerializeTupleStruct for TupleStructEncoder<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.0)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Encodes a tuple variant to binary.
pub struct TupleVariantEncoder<'a, W>(&'a mut Encoder<W>)
where
    W: Write;

impl<'a, W> TupleVariantEncoder<'a, W>
where
    W: Write,
{
    /// Creates a new tuple variant encoder.
    pub fn new(
        encoder: &'a mut Encoder<W>,
        name: &'static str,
        variant_index: u32,
    ) -> crate::Result<Self> {
        if variant_index < 256 {
            encoder.0.write_all(&(variant_index as u8).to_be_bytes())?;
            Ok(Self(encoder))
        } else {
            Err(Error::TooManyVariants(name))
        }
    }
}

impl<'a, W> SerializeTupleVariant for TupleVariantEncoder<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.0)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Encodes a map to binary.
pub struct MapEncoder<'a, W>(&'a mut Encoder<W>)
where
    W: Write;

impl<'a, W> MapEncoder<'a, W>
where
    W: Write,
{
    /// Creates a new map encoder.
    pub fn new(encoder: &'a mut Encoder<W>, len: usize) -> crate::Result<Self> {
        let encoded_len = encode_len_large(len);
        encoder.0.write_all(&encoded_len)?;
        Ok(Self(encoder))
    }
}

impl<'a, W> SerializeMap for MapEncoder<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        key.serialize(&mut *self.0)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.0)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Encodes a struct to binary.
pub struct StructEncoder<'a, W>(&'a mut Encoder<W>)
where
    W: Write;

impl<'a, W> StructEncoder<'a, W>
where
    W: Write,
{
    /// Creates a new struct encoder.
    pub fn new(encoder: &'a mut Encoder<W>) -> Self {
        Self(encoder)
    }
}

impl<'a, W> SerializeStruct for StructEncoder<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.0)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        Err(Error::FieldSkippingNotAllowed(key))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Encodes a struct variant to binary.
pub struct StructVariantEncoder<'a, W>(&'a mut Encoder<W>)
where
    W: Write;

impl<'a, W> StructVariantEncoder<'a, W>
where
    W: Write,
{
    /// Creates a new struct variant encoder.
    pub fn new(
        encoder: &'a mut Encoder<W>,
        name: &'static str,
        variant_index: u32,
    ) -> crate::Result<Self> {
        if variant_index < 256 {
            encoder.0.write_all(&(variant_index as u8).to_be_bytes())?;
            Ok(Self(encoder))
        } else {
            Err(Error::TooManyVariants(name))
        }
    }
}

impl<'a, W> SerializeStructVariant for StructVariantEncoder<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(&mut *self.0)
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        Err(Error::FieldSkippingNotAllowed(key))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
