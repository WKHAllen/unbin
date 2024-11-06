//! I/O reading.

use crate::util::*;
use crate::{Error, Result};
use serde::de::Visitor;
use std::io::{self, Write};

/// Trait to allow reading bytes. Similar to [`std::io::Read`], but also
/// supports reading from byte arrays.
pub trait Read<'de> {
    /// Reads the exact number of bytes required to fill buffer.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;

    /// Reads a string from the reader and passes it to the visitor.
    fn visit_str<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;

    /// Reads a byte slice from the reader and passes it to the visitor.
    fn visit_bytes<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;

    /// Reads `N` bytes from the underlying reader into a `[u8; N]`.
    fn read_n_array<const N: usize>(&mut self) -> crate::Result<[u8; N]> {
        let mut bytes = [0; N];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    /// Reads `n` bytes from the underlying reader into a `Vec<u8>`.
    fn read_n_vec(&mut self, n: usize) -> crate::Result<Vec<u8>> {
        let mut bytes = vec![0; n];
        self.read_exact(&mut bytes)?;
        Ok(bytes)
    }

    /// Reads and returns a dynamically sized collection of bytes, assuming they
    /// are encoded with a small length.
    fn read_bytes_with_small_len(&mut self) -> crate::Result<Vec<u8>> {
        let len = self.read_n_array::<1>()?;
        let decoded_len = decode_len_small(len[0]);
        self.read_n_vec(decoded_len)
    }

    /// Reads and returns a dynamically sized collection of bytes, assuming they
    /// are encoded with a large length.
    fn read_bytes_with_large_len(&mut self) -> crate::Result<Vec<u8>> {
        let len1 = self.read_n_array::<1>()?;
        let decoded_len1 = decode_len_small(len1[0]);
        let len2 = self.read_n_vec(decoded_len1)?;
        let decoded_len2 = decode_len_large(&len2);
        self.read_n_vec(decoded_len2)
    }
}

impl<'de, R> Read<'de> for R
where
    R: io::Read,
{
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(io::Read::read_exact(self, buf)?)
    }

    fn visit_str<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.read_bytes_with_large_len()?;
        let string = std::str::from_utf8(&bytes)?;
        visitor.visit_str(string)
    }

    fn visit_bytes<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let bytes = self.read_bytes_with_large_len()?;
        visitor.visit_bytes(&bytes)
    }
}

/// A wrapper around a [`Read`]-able byte array.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BytesReader<'a> {
    /// The byte buffer.
    bytes: &'a [u8],
}

impl<'a> BytesReader<'a> {
    /// Constructs a new reader from a byte array.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// Returns the full buffer as a slice.
    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[u8] {
        self.bytes
    }

    /// Reads and returns a slice containing the requested number of bytes.
    pub fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if len <= self.bytes.len() {
            let (first, rest) = self.bytes.split_at(len);
            self.bytes = rest;
            Ok(first)
        } else {
            Err(Error::UnexpectedEof)
        }
    }
}

impl<'de, 'a> Read<'de> for BytesReader<'a>
where
    'a: 'de,
{
    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        let num_bytes = buf.write(self.bytes)?;
        self.bytes = &self.bytes[num_bytes..];
        Ok(())
    }

    fn visit_str<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len1 = self.read_n_array::<1>()?;
        let decoded_len1 = decode_len_small(len1[0]);
        let len2 = self.read_n_vec(decoded_len1)?;
        let decoded_len2 = decode_len_large(&len2);
        let bytes = self.read_bytes(decoded_len2)?;
        let string = std::str::from_utf8(bytes)?;
        visitor.visit_borrowed_str(string)
    }

    fn visit_bytes<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len1 = self.read_n_array::<1>()?;
        let decoded_len1 = decode_len_small(len1[0]);
        let len2 = self.read_n_vec(decoded_len1)?;
        let decoded_len2 = decode_len_large(&len2);
        let bytes = self.read_bytes(decoded_len2)?;
        visitor.visit_borrowed_bytes(bytes)
    }
}
