// File: Linux-Server/src/protocol/bap_frame.rs
// Title: BAP Outer Stream Framing
// RFC Reference: RFC 793 Section 2.1 (TCP Wire Protocols)
// Plain English: Encodes and decodes BAP0 frames over a byte stream.

use crate::encoding::byte_order::*;
use crate::error::{Result, SunriseError};
use crate::protocol::opcode::Opcode;

pub const BAP_MAGIC: [u8; 4] = [0x42, 0x41, 0x50, 0x30]; // "BAP0"
pub const HEADER_SIZE: usize = 14; // Magic(4) + Length(4) + TxId(4) + Opcode(2)
pub const MAX_PAYLOAD_SIZE: usize = 16 * 1024 * 1024; // 16MB max limit

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BapFrame {
    pub transaction_id: u32,
    pub opcode: Opcode,
    pub payload: Vec<u8>,
}

impl BapFrame {
    pub fn new(transaction_id: u32, opcode: Opcode, payload: Vec<u8>) -> Self {
        Self {
            transaction_id,
            opcode,
            payload,
        }
    }

    pub fn encoded_size(&self) -> usize {
        HEADER_SIZE + self.payload.len()
    }

    pub fn encode(&self, target: &mut [u8]) -> Result<usize> {
        let needed = self.encoded_size();
        if target.len() < needed {
            return Err(SunriseError::BufferTooShort {
                needed,
                available: target.len(),
            });
        }
        if self.payload.len() > MAX_PAYLOAD_SIZE {
            return Err(SunriseError::PayloadTooLarge {
                length: self.payload.len(),
                max: MAX_PAYLOAD_SIZE,
            });
        }

        target[..4].copy_from_slice(&BAP_MAGIC);
        let total_body_len = (self.payload.len() + 6) as u32; // TxId(4) + Opcode(2) + Payload
        write_u32_be(&mut target[4..8], total_body_len);
        write_u32_be(&mut target[8..12], self.transaction_id);
        write_u16_be(&mut target[12..14], self.opcode.into());
        target[14..needed].copy_from_slice(&self.payload);

        Ok(needed)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; self.encoded_size()];
        self.encode(&mut buf)?;
        Ok(buf)
    }

    pub fn decode(source: &[u8]) -> Result<(Self, usize)> {
        if source.len() < HEADER_SIZE {
            return Err(SunriseError::BufferTooShort {
                needed: HEADER_SIZE,
                available: source.len(),
            });
        }

        if source[..4] != BAP_MAGIC {
            let mut got = [0u8; 4];
            got.copy_from_slice(&source[..4]);
            return Err(SunriseError::InvalidMagicNumber(got));
        }

        let body_len = read_u32_be(&source[4..8]).ok_or(SunriseError::BufferTooShort {
            needed: 8,
            available: source.len(),
        })? as usize;

        if body_len < 6 {
            return Err(SunriseError::BufferTooShort {
                needed: 6,
                available: body_len,
            });
        }

        let payload_len = body_len - 6;
        if payload_len > MAX_PAYLOAD_SIZE {
            return Err(SunriseError::PayloadTooLarge {
                length: payload_len,
                max: MAX_PAYLOAD_SIZE,
            });
        }

        let total_frame_len = 8 + body_len;
        if source.len() < total_frame_len {
            return Err(SunriseError::BufferTooShort {
                needed: total_frame_len,
                available: source.len(),
            });
        }

        let tx_id = read_u32_be(&source[8..12]).unwrap();
        let raw_opcode = read_u16_be(&source[12..14]).unwrap();
        let payload = source[14..total_frame_len].to_vec();

        Ok((
            Self {
                transaction_id: tx_id,
                opcode: Opcode::from(raw_opcode),
                payload,
            },
            total_frame_len,
        ))
    }
}
