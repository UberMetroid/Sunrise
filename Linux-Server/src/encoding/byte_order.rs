// File: Linux-Server/src/encoding/byte_order.rs
// Title: Network Byte Order Converters
// RFC Reference: RFC 793 Section 2.1 (Transmission Control Protocol - Big Endian Wire Format)
// Plain English: Converts 16-bit, 32-bit, and 64-bit numbers between native and network order.

pub fn read_u16_be(slice: &[u8]) -> Option<u16> {
    if slice.len() < 2 {
        return None;
    }
    let array = [slice[0], slice[1]];
    Some(u16::from_be_bytes(array))
}

pub fn read_u32_be(slice: &[u8]) -> Option<u32> {
    if slice.len() < 4 {
        return None;
    }
    let array = [slice[0], slice[1], slice[2], slice[3]];
    Some(u32::from_be_bytes(array))
}

pub fn read_u64_be(slice: &[u8]) -> Option<u64> {
    if slice.len() < 8 {
        return None;
    }
    let mut array = [0u8; 8];
    array.copy_from_slice(&slice[..8]);
    Some(u64::from_be_bytes(array))
}

pub fn write_u16_be(target: &mut [u8], value: u16) -> bool {
    if target.len() < 2 {
        return false;
    }
    target[..2].copy_from_slice(&value.to_be_bytes());
    true
}

pub fn write_u32_be(target: &mut [u8], value: u32) -> bool {
    if target.len() < 4 {
        return false;
    }
    target[..4].copy_from_slice(&value.to_be_bytes());
    true
}

pub fn write_u64_be(target: &mut [u8], value: u64) -> bool {
    if target.len() < 8 {
        return false;
    }
    target[..8].copy_from_slice(&value.to_be_bytes());
    true
}

pub fn read_u16_le(slice: &[u8]) -> Option<u16> {
    if slice.len() < 2 {
        return None;
    }
    let array = [slice[0], slice[1]];
    Some(u16::from_le_bytes(array))
}

pub fn read_u32_le(slice: &[u8]) -> Option<u32> {
    if slice.len() < 4 {
        return None;
    }
    let array = [slice[0], slice[1], slice[2], slice[3]];
    Some(u32::from_le_bytes(array))
}

pub fn write_u32_le(target: &mut [u8], value: u32) -> bool {
    if target.len() < 4 {
        return false;
    }
    target[..4].copy_from_slice(&value.to_le_bytes());
    true
}
