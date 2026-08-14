// File: linux/src/encoding/bit_stream.rs
// Title: Bit-level Reader and Writer
// Plain English: Serializes and deserializes arbitrary bit fields into a byte buffer.

use crate::error::{Result, SunriseError};

pub struct BitWriter<'a> {
    buffer: &'a mut [u8],
    bit_cursor: usize,
}

impl<'a> BitWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        for b in buffer.iter_mut() {
            *b = 0;
        }
        Self { buffer, bit_cursor: 0 }
    }

    pub fn write_bits(&mut self, value: u64, bit_count: usize) -> Result<()> {
        if bit_count == 0 {
            return Ok(());
        }
        if bit_count > 64 {
            return Err(SunriseError::InvalidBitStream);
        }
        let total_bits_needed = self.bit_cursor + bit_count;
        let bytes_needed = (total_bits_needed + 7) / 8;
        if bytes_needed > self.buffer.len() {
            return Err(SunriseError::BufferTooShort {
                needed: bytes_needed,
                available: self.buffer.len(),
            });
        }

        for i in (0..bit_count).rev() {
            let bit = ((value >> i) & 1) as u8;
            let byte_idx = self.bit_cursor / 8;
            let bit_idx = 7 - (self.bit_cursor % 8);
            if bit == 1 {
                self.buffer[byte_idx] |= 1 << bit_idx;
            }
            self.bit_cursor += 1;
        }
        Ok(())
    }

    pub fn finish(self) -> usize {
        (self.bit_cursor + 7) / 8
    }

    pub fn total_bits(&self) -> usize {
        self.bit_cursor
    }
}

pub struct BitReader<'a> {
    buffer: &'a [u8],
    bit_cursor: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, bit_cursor: 0 }
    }

    pub fn read_bits(&mut self, bit_count: usize) -> Result<u64> {
        if bit_count == 0 {
            return Ok(0);
        }
        if bit_count > 64 {
            return Err(SunriseError::InvalidBitStream);
        }
        let total_bits_needed = self.bit_cursor + bit_count;
        let bytes_needed = (total_bits_needed + 7) / 8;
        if bytes_needed > self.buffer.len() {
            return Err(SunriseError::BufferTooShort {
                needed: bytes_needed,
                available: self.buffer.len(),
            });
        }

        let mut result: u64 = 0;
        for _ in 0..bit_count {
            let byte_idx = self.bit_cursor / 8;
            let bit_idx = 7 - (self.bit_cursor % 8);
            let bit = (self.buffer[byte_idx] >> bit_idx) & 1;
            result = (result << 1) | (bit as u64);
            self.bit_cursor += 1;
        }
        Ok(result)
    }

    pub fn remaining_bits(&self) -> usize {
        (self.buffer.len() * 8).saturating_sub(self.bit_cursor)
    }
}
