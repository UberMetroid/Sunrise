// File: Linux-Server/src/encoding/varint.rs
// Title: Protobuf Varint Encoding and Decoding
// Plain English: Reads and writes variable-length integers (1 to 10 bytes).
// Boundary Safety: Prevents integer overflow and buffer overruns.

use crate::error::{Result, SunriseError};

pub const MAX_VARINT_BYTES: usize = 10;

pub fn encode_varint(mut value: u64, target: &mut [u8]) -> Result<usize> {
    let mut written = 0;
    loop {
        if written >= target.len() {
            return Err(SunriseError::BufferTooShort {
                needed: written + 1,
                available: target.len(),
            });
        }
        let mut byte = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
            target[written] = byte;
            written += 1;
        } else {
            target[written] = byte;
            written += 1;
            break;
        }
    }
    Ok(written)
}

pub fn decode_varint(source: &[u8]) -> Result<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0;
    let mut read = 0;

    for &byte in source {
        read += 1;
        if read > MAX_VARINT_BYTES {
            return Err(SunriseError::CorruptVarint);
        }
        let val = (byte & 0x7F) as u64;
        if shift >= 64 && val != 0 {
            return Err(SunriseError::CorruptVarint);
        }
        if shift == 63 && val > 1 {
            return Err(SunriseError::CorruptVarint);
        }
        result |= val << shift;
        if (byte & 0x80) == 0 {
            return Ok((result, read));
        }
        shift += 7;
    }

    Err(SunriseError::BufferTooShort {
        needed: read + 1,
        available: source.len(),
    })
}

pub fn varint_size(mut value: u64) -> usize {
    let mut size = 0;
    loop {
        size += 1;
        value >>= 7;
        if value == 0 {
            break;
        }
    }
    size
}

pub fn encode_tag(field_number: u32, wire_type: u8, target: &mut [u8]) -> Result<usize> {
    let tag = ((field_number as u64) << 3) | ((wire_type as u64) & 0x07);
    encode_varint(tag, target)
}

pub fn decode_tag(source: &[u8]) -> Result<((u32, u8), usize)> {
    let (tag, read) = decode_varint(source)?;
    let field_number = (tag >> 3) as u32;
    let wire_type = (tag & 0x07) as u8;
    Ok(((field_number, wire_type), read))
}
