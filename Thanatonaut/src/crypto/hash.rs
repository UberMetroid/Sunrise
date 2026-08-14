// File: Thanatonaut/src/crypto/hash.rs
// Title: Cryptographic Hashes & HMAC
// RFC Reference: RFC 6234 (US Secure Hash Algorithms - SHA-256), RFC 2104 (HMAC)
// Plain English: Computes SHA-256 digests and HMAC authentication tags for manifests and tokens.

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use crate::error::Result;

pub const SHA256_DIGEST_SIZE: usize = 32;

pub fn sha256_digest(data: &[u8]) -> [u8; SHA256_DIGEST_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut output = [0u8; SHA256_DIGEST_SIZE];
    output.copy_from_slice(&result);
    output
}

pub fn sha256_hex(data: &[u8]) -> String {
    let digest = sha256_digest(data);
    let mut hex = String::with_capacity(SHA256_DIGEST_SIZE * 2);
    for byte in digest {
        use std::fmt::Write;
        let _ = write!(hex, "{:02x}", byte);
    }
    hex
}

pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<[u8; SHA256_DIGEST_SIZE]> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|_| crate::error::SunriseError::InvalidKeyLength {
            expected: 32,
            got: key.len(),
        })?;
    mac.update(data);
    let result = mac.finalize().into_bytes();
    let mut output = [0u8; SHA256_DIGEST_SIZE];
    output.copy_from_slice(&result);
    Ok(output)
}
