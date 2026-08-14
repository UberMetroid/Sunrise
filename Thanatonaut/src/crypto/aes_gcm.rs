// File: Thanatonaut/src/crypto/aes_gcm.rs
// Title: Authenticated Encryption with Associated Data (AEAD) AES-GCM
// RFC Reference: RFC 5116 Section 5.1 (AEAD_AES_128_GCM)
// Plain English: Encrypts and decrypts frames with 128-bit key, 96-bit nonce, and 128-bit tag.

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes128Gcm, Key, Nonce};
use crate::error::{Result, SunriseError};

pub const KEY_SIZE: usize = 16;
pub const NONCE_SIZE: usize = 12;
pub const TAG_SIZE: usize = 16;

pub fn encrypt_aes_gcm(
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
    associated_data: &[u8],
) -> Result<(Vec<u8>, [u8; TAG_SIZE])> {
    if key.len() != KEY_SIZE {
        return Err(SunriseError::InvalidKeyLength {
            expected: KEY_SIZE,
            got: key.len(),
        });
    }
    if nonce.len() != NONCE_SIZE {
        return Err(SunriseError::InvalidNonceLength {
            expected: NONCE_SIZE,
            got: nonce.len(),
        });
    }

    let cipher = Aes128Gcm::new(Key::<Aes128Gcm>::from_slice(key));
    let gcm_nonce = Nonce::from_slice(nonce);

    let payload = Payload {
        msg: plaintext,
        aad: associated_data,
    };

    let ciphertext_with_tag = cipher
        .encrypt(gcm_nonce, payload)
        .map_err(|e| SunriseError::IoError(format!("AES-GCM encrypt failed: {}", e)))?;

    if ciphertext_with_tag.len() < TAG_SIZE {
        return Err(SunriseError::DecryptionFailed);
    }

    let split_pos = ciphertext_with_tag.len() - TAG_SIZE;
    let ciphertext = ciphertext_with_tag[..split_pos].to_vec();
    let mut tag = [0u8; TAG_SIZE];
    tag.copy_from_slice(&ciphertext_with_tag[split_pos..]);

    Ok((ciphertext, tag))
}

pub fn decrypt_aes_gcm(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    tag: &[u8],
    associated_data: &[u8],
) -> Result<Vec<u8>> {
    if key.len() != KEY_SIZE {
        return Err(SunriseError::InvalidKeyLength {
            expected: KEY_SIZE,
            got: key.len(),
        });
    }
    if nonce.len() != NONCE_SIZE {
        return Err(SunriseError::InvalidNonceLength {
            expected: NONCE_SIZE,
            got: nonce.len(),
        });
    }
    if tag.len() != TAG_SIZE {
        return Err(SunriseError::InvalidTagLength {
            expected: TAG_SIZE,
            got: tag.len(),
        });
    }

    let cipher = Aes128Gcm::new(Key::<Aes128Gcm>::from_slice(key));
    let gcm_nonce = Nonce::from_slice(nonce);

    let mut combined = Vec::with_capacity(ciphertext.len() + tag.len());
    combined.extend_from_slice(ciphertext);
    combined.extend_from_slice(tag);

    let payload = Payload {
        msg: &combined,
        aad: associated_data,
    };

    cipher
        .decrypt(gcm_nonce, payload)
        .map_err(|_| SunriseError::DecryptionFailed)
}
