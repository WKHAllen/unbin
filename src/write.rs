//! I/O writing.

use crate::Result;
use std::io;

/// Trait to allow writing bytes. Similar to [`std::io::Write`], but also
/// supports writing to byte arrays.
pub trait Write {
    /// Writes the entire buffer to the writer.
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;

    /// Flushes this output stream, ensuring that all intermediately buffered
    /// contents reach their destination.
    fn flush(&mut self) -> Result<()>;
}

impl<W> Write for W
where
    W: io::Write,
{
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        Ok(io::Write::write_all(self, buf)?)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(io::Write::flush(self)?)
    }
}

/// A wrapper around a [`Write`]-able byte array.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BytesWriter {
    /// The byte buffer.
    bytes: Vec<u8>,
}

impl BytesWriter {
    /// Constructs a new writer with an empty byte array.
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// Returns the full buffer as a slice.
    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the full buffer as a mutable slice.
    #[allow(dead_code)]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Unwraps and returns the full inner buffer.
    pub fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BytesWriter {
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        io::Write::write(&mut self.bytes, buf)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        io::Write::flush(&mut self.bytes)?;
        Ok(())
    }
}
