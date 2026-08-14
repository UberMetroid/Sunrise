// File: linux/src/error.rs
// Title: Sunrise Error Types
// Plain English: Defines all failure cases for Sunrise Linux components.
// Fail-Closed: Every error is explicit and typed to prevent torn state.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SunriseError {
    // Framing errors
    InvalidMagicNumber([u8; 4]),
    BufferTooShort { needed: usize, available: usize },
    PayloadTooLarge { length: usize, max: usize },
    InvalidTransactionId(u32),

    // Encoding & Protobuf errors
    CorruptVarint,
    InvalidBitStream,
    InvalidWireType(u8),
    MissingRequiredField(&'static str),

    // Cryptography errors (RFC 5116 / RFC 6234)
    DecryptionFailed,
    InvalidKeyLength { expected: usize, got: usize },
    InvalidNonceLength { expected: usize, got: usize },
    InvalidTagLength { expected: usize, got: usize },

    // Game state & Calculation errors
    InvalidBucketCount(usize),
    CharacterNotFound(u64),
    ItemNotFound(u32),

    // Network & Server errors (RFC 793)
    IoError(String),
    ConnectionClosed,
    AddressInUse(String),

    // Settings errors (RFC 8259)
    InvalidJson(String),
    FileNotFound(String),
}

impl fmt::Display for SunriseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagicNumber(m) => {
                write!(f, "Invalid magic number: {:02X?}", m)
            }
            Self::BufferTooShort { needed, available } => {
                write!(f, "Buffer too short: need {} bytes, got {}", needed, available)
            }
            Self::PayloadTooLarge { length, max } => {
                write!(f, "Payload too large: {} bytes exceeds max {}", length, max)
            }
            Self::InvalidTransactionId(id) => {
                write!(f, "Invalid transaction id: {}", id)
            }
            Self::CorruptVarint => write!(f, "Corrupted protobuf varint encoding"),
            Self::InvalidBitStream => write!(f, "Invalid bit stream read/write operation"),
            Self::InvalidWireType(w) => write!(f, "Invalid protobuf wire type: {}", w),
            Self::MissingRequiredField(name) => {
                write!(f, "Missing required message field: {}", name)
            }
            Self::DecryptionFailed => write!(f, "AEAD decryption failed or authentication tag mismatch"),
            Self::InvalidKeyLength { expected, got } => {
                write!(f, "Invalid key length: expected {}, got {}", expected, got)
            }
            Self::InvalidNonceLength { expected, got } => {
                write!(f, "Invalid nonce length: expected {}, got {}", expected, got)
            }
            Self::InvalidTagLength { expected, got } => {
                write!(f, "Invalid tag length: expected {}, got {}", expected, got)
            }
            Self::InvalidBucketCount(c) => {
                write!(f, "Invalid gear bucket count for light level: {}", c)
            }
            Self::CharacterNotFound(id) => write!(f, "Character not found: {}", id),
            Self::ItemNotFound(id) => write!(f, "Item definition not found: {}", id),
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
            Self::ConnectionClosed => write!(f, "Connection closed by remote peer"),
            Self::AddressInUse(addr) => write!(f, "Address already in use: {}", addr),
            Self::InvalidJson(msg) => write!(f, "Invalid JSON: {}", msg),
            Self::FileNotFound(path) => write!(f, "File not found: {}", path),
        }
    }
}

impl std::error::Error for SunriseError {}

impl From<std::io::Error> for SunriseError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, SunriseError>;
