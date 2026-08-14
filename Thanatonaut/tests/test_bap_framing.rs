// File: linux/tests/test_bap_framing.rs
// Title: BAP Framing E-M Verification Proofs
// RFC Reference: RFC 793 Section 2.1 (TCP Wire Framing)
// Plain English: Validates BAP0 outer framing, header serialization, and boundary rejection.

use thanatonaut::error::SunriseError;
use thanatonaut::protocol::bap_frame::{BapFrame, BAP_MAGIC, HEADER_SIZE};
use thanatonaut::protocol::opcode::Opcode;

#[test]
fn test_bap_frame_roundtrip() {
    let payload = vec![0x01, 0x02, 0x03, 0x04, 0x05];
    let frame = BapFrame::new(1337, Opcode::AccountSummary, payload.clone());

    let encoded = frame.to_bytes().expect("Encoding should succeed");
    assert_eq!(encoded.len(), HEADER_SIZE + payload.len());
    assert_eq!(&encoded[..4], &BAP_MAGIC);

    let (decoded, consumed) = BapFrame::decode(&encoded).expect("Decoding should succeed");
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.transaction_id, 1337);
    assert_eq!(decoded.opcode, Opcode::AccountSummary);
    assert_eq!(decoded.payload, payload);
}

#[test]
fn test_bap_frame_empty_payload() {
    let frame = BapFrame::new(100, Opcode::Signon, Vec::new());
    let encoded = frame.to_bytes().expect("Encoding empty payload should succeed");
    assert_eq!(encoded.len(), HEADER_SIZE);

    let (decoded, consumed) = BapFrame::decode(&encoded).expect("Decoding empty payload should succeed");
    assert_eq!(consumed, HEADER_SIZE);
    assert_eq!(decoded.transaction_id, 100);
    assert_eq!(decoded.opcode, Opcode::Signon);
    assert!(decoded.payload.is_empty());
}

#[test]
fn test_bap_frame_invalid_magic_rejected() {
    let bad_frame = vec![0x58, 0x58, 0x58, 0x58, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01];
    let err = BapFrame::decode(&bad_frame).unwrap_err();
    match err {
        SunriseError::InvalidMagicNumber(m) => assert_eq!(m, [0x58, 0x58, 0x58, 0x58]),
        other => panic!("Expected InvalidMagicNumber, got: {:?}", other),
    }
}

#[test]
fn test_bap_frame_truncated_buffer_rejected() {
    let truncated = vec![0x42, 0x41, 0x50, 0x30, 0x00, 0x00];
    let err = BapFrame::decode(&truncated).unwrap_err();
    match err {
        SunriseError::BufferTooShort { needed, available } => {
            assert_eq!(needed, HEADER_SIZE);
            assert_eq!(available, 6);
        }
        other => panic!("Expected BufferTooShort, got: {:?}", other),
    }
}
