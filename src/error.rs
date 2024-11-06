//! Library error types.

use std::io;
use thiserror::Error;

/// All types of serializable values.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Char,
    Str,
    String,
    Bytes,
    ByteBuf,
    Option,
    Unit,
    UnitStruct,
    NewtypeStruct,
    Seq,
    Tuple,
    TupleStruct,
    Map,
    Struct,
    Enum,
}

/// Library-level error.
#[derive(Debug, Error)]
pub enum Error {
    /// Sequences of unknown length are not allowed.
    #[error("sequences of unknown length are not allowed")]
    UnknownSeqLengthNotAllowed,
    /// Maps of unknown length are not allowed.
    #[error("maps of unknown length are not allowed")]
    UnknownMapLengthNotAllowed,
    /// An enum has more than 256 variants.
    #[error("enum `{0}` has more than 256 variants")]
    TooManyVariants(&'static str),
    /// The deserializer is trying to use `deserialize_any`.
    #[error("`deserialize_any` is not allowed")]
    CannotDeserializeAny,
    /// The deserializer is trying to use `deserialize_identifier`.
    #[error("`deserialize_identifier` is not allowed")]
    CannotDeserializeIdentifier,
    /// A byte reader reached the end of the stream prematurely.
    #[error("a byte reader reached the end of the stream prematurely")]
    UnexpectedEof,
    /// An invalid byte sequence was encountered.
    #[error("invalid byte sequence while deserializing value of type `{ty:?}`: `{bytes:?}`")]
    InvalidBytes {
        /// The type of value where the deserializer failed.
        ty: ValueType,
        /// The sequence of invalid bytes.
        bytes: Vec<u8>,
    },
    /// An I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    /// A UTF-8 encode/decode error.
    #[error("UTF-8 encode/decode error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    /// A custom error message from `serde`.
    #[error("serialization error: {0}")]
    Custom(String),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

/// Library-level `Result` alias.
pub type Result<T> = core::result::Result<T, Error>;
