// File: linux/tests/test_crypto.rs
// Title: Cryptographic E-M Verification Proofs
// RFC References: RFC 5116 (AES-GCM), RFC 6234 (SHA-256), RFC 2104 (HMAC)
// Plain English: Validates authenticated encryption, decryption, SHA-256 digests, and HMAC tags.

use sunrise_linux::crypto::aes_gcm::{decrypt_aes_gcm, encrypt_aes_gcm};
use sunrise_linux::crypto::hash::{hmac_sha256, sha256_hex};

#[test]
fn test_sha256_known_vector() {
    // Known NIST vector: sha256("")
    let empty_hex = sha256_hex(b"");
    assert_eq!(
        empty_hex,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );

    // Known NIST vector: sha256("abc")
    let abc_hex = sha256_hex(b"abc");
    assert_eq!(
        abc_hex,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn test_hmac_sha256_known_vector() {
    let key = [0x0b; 20];
    let data = b"Hi There";
    let tag = hmac_sha256(&key, data).expect("HMAC should compute");
    let mut hex = String::new();
    for b in tag {
        use std::fmt::Write;
        let _ = write!(hex, "{:02x}", b);
    }
    // RFC 4231 Test Case 1
    assert_eq!(
        hex,
        "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
    );
}

#[test]
fn test_aes_gcm_roundtrip() {
    let key = [0x42u8; 16];
    let nonce = [0x24u8; 12];
    let plaintext = b"Project Sunrise Linux Native Security Layer";
    let aad = b"BAP0-AUTH-DATA";

    let (ciphertext, tag) = encrypt_aes_gcm(&key, &nonce, plaintext, aad)
        .expect("Encryption should succeed");

    assert_eq!(ciphertext.len(), plaintext.len());
    assert_eq!(tag.len(), 16);

    let decrypted = decrypt_aes_gcm(&key, &nonce, &ciphertext, &tag, aad)
        .expect("Decryption should succeed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_aes_gcm_corrupted_tag_fails() {
    let key = [0x42u8; 16];
    let nonce = [0x24u8; 12];
    let plaintext = b"Test message";
    let aad = b"AAD";

    let (ciphertext, mut tag) = encrypt_aes_gcm(&key, &nonce, plaintext, aad).unwrap();
    // Tamper with tag
    tag[0] ^= 0xFF;

    assert!(decrypt_aes_gcm(&key, &nonce, &ciphertext, &tag, aad).is_err());
}
