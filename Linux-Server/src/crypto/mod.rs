// File: Linux-Server/src/crypto/mod.rs
// Title: Cryptography Module
// RFC Reference: RFC 5116 (AEAD), RFC 6234 (SHA-256), RFC 2104 (HMAC)
// Plain English: Provides authenticated symmetric decryption/encryption and cryptographic hashing.

pub mod aes_gcm;
pub mod hash;

pub use aes_gcm::*;
pub use hash::*;
