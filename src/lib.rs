#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "README.md" ) ) ]
#![deny(missing_docs)]
use core::fmt;

/// Error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// All I/O errors
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// Unsupported data type
    #[error("unsupported data type: {0}")]
    DataType(u8),
    /// Unsupported COT
    #[error("unsupported COT: {0}")]
    COT(u8),
    /// Operation overflow
    #[error("operation overflow")]
    Overflow,
    /// Data conversion errors
    #[error("conversion failed: {0}")]
    Conversion(String),
    /// TX/RX chat sequence errors
    #[error("chat sequence error ({0}/{1}")]
    ChatSequence(u16, u16),
    /// Invalid data
    #[error("invalid data: {0}")]
    InvalidData(String),
}

impl Error {
    fn conversion(msg: impl fmt::Display) -> Self {
        Error::Conversion(msg.to_string())
    }
    fn invalid_data(msg: impl fmt::Display) -> Self {
        Error::InvalidData(msg.to_string())
    }
}

/// Server events
pub mod events;
/// IEC 60870-5-101
pub mod telegram101;
/// IEC 60870-5-104
pub mod telegram104;
/// Common data types
pub mod types;
