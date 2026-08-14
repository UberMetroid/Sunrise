// File: Thanatonaut/src/protocol/envelope.rs
// Title: Web Service Envelope Codec
// RFC Reference: RFC 9110 (HTTP Semantics & Envelope Status Encodings)
// Plain English: Serializes standard response wrappers with status codes and version stamps.

use crate::encoding::byte_order::*;
use crate::error::{Result, SunriseError};
use crate::protocol::opcode::Opcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatusCode {
    Success = 1,
    MalformedRequest = 2,
    Unauthorized = 3,
    InternalError = 4,
    ItemLocked = 5,
    InvalidBucket = 6,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceResponseEnvelope {
    pub opcode: Opcode,
    pub transaction_id: u32,
    pub status: ServiceStatusCode,
    pub version: u32,
    pub payload: Vec<u8>,
}

impl ServiceResponseEnvelope {
    pub fn new(opcode: Opcode, transaction_id: u32, status: ServiceStatusCode, version: u32, payload: Vec<u8>) -> Self {
        Self {
            opcode,
            transaction_id,
            status,
            version,
            payload,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        // Format: Opcode (2) + TxId (4) + StatusCode (4) + Version (4) + Payload (N)
        let total_size = 14 + self.payload.len();
        let mut buffer = vec![0u8; total_size];

        write_u16_be(&mut buffer[0..2], self.opcode.into());
        write_u32_be(&mut buffer[2..6], self.transaction_id);
        write_u32_be(&mut buffer[6..10], self.status as u32);
        write_u32_be(&mut buffer[10..14], self.version);
        buffer[14..].copy_from_slice(&self.payload);

        Ok(buffer)
    }

    pub fn decode(source: &[u8]) -> Result<Self> {
        if source.len() < 14 {
            return Err(SunriseError::BufferTooShort {
                needed: 14,
                available: source.len(),
            });
        }

        let raw_opcode = read_u16_be(&source[0..2]).unwrap();
        let tx_id = read_u32_be(&source[2..6]).unwrap();
        let raw_status = read_u32_be(&source[6..10]).unwrap();
        let version = read_u32_be(&source[10..14]).unwrap();
        let payload = source[14..].to_vec();

        let status = match raw_status {
            1 => ServiceStatusCode::Success,
            2 => ServiceStatusCode::MalformedRequest,
            3 => ServiceStatusCode::Unauthorized,
            5 => ServiceStatusCode::ItemLocked,
            6 => ServiceStatusCode::InvalidBucket,
            _ => ServiceStatusCode::InternalError,
        };

        Ok(Self {
            opcode: Opcode::from(raw_opcode),
            transaction_id: tx_id,
            status,
            version,
            payload,
        })
    }
}
