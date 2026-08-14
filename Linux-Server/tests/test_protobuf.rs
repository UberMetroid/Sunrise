// File: linux/tests/test_protobuf.rs
// Title: Protobuf & Varint E-M Verification Proofs
// Plain English: Tests varint encoding/decoding, bit streams, and overflow safety.

use sunrise_linux::encoding::bit_stream::{BitReader, BitWriter};
use sunrise_linux::encoding::varint::{decode_tag, decode_varint, encode_tag, encode_varint, varint_size};

#[test]
fn test_varint_single_byte() {
    let mut buf = [0u8; 10];
    let written = encode_varint(1, &mut buf).unwrap();
    assert_eq!(written, 1);
    assert_eq!(buf[0], 0x01);

    let (val, read) = decode_varint(&buf[..written]).unwrap();
    assert_eq!(val, 1);
    assert_eq!(read, 1);
}

#[test]
fn test_varint_multi_byte_300() {
    // 300 = 0xAC 0x02 in standard protobuf varint
    let mut buf = [0u8; 10];
    let written = encode_varint(300, &mut buf).unwrap();
    assert_eq!(written, 2);
    assert_eq!(buf[0], 0xAC);
    assert_eq!(buf[1], 0x02);

    let (val, read) = decode_varint(&buf[..written]).unwrap();
    assert_eq!(val, 300);
    assert_eq!(read, 2);
}

#[test]
fn test_varint_max_u64() {
    let mut buf = [0u8; 10];
    let written = encode_varint(u64::MAX, &mut buf).unwrap();
    assert_eq!(written, 10);

    let (val, read) = decode_varint(&buf[..written]).unwrap();
    assert_eq!(val, u64::MAX);
    assert_eq!(read, 10);
    assert_eq!(varint_size(u64::MAX), 10);
}

#[test]
fn test_protobuf_tag_encoding() {
    let mut buf = [0u8; 10];
    // Field 1, wire type 2 (length-delimited) -> (1 << 3) | 2 = 10 (0x0A)
    let written = encode_tag(1, 2, &mut buf).unwrap();
    assert_eq!(written, 1);
    assert_eq!(buf[0], 0x0A);

    let ((field_num, wire_type), read) = decode_tag(&buf[..written]).unwrap();
    assert_eq!(field_num, 1);
    assert_eq!(wire_type, 2);
    assert_eq!(read, 1);
}

#[test]
fn test_bit_stream_read_write() {
    let mut buffer = [0u8; 4];
    {
        let mut writer = BitWriter::new(&mut buffer);
        writer.write_bits(0b101, 3).unwrap();
        writer.write_bits(0b1100, 4).unwrap();
        writer.write_bits(0b1, 1).unwrap();
        assert_eq!(writer.total_bits(), 8);
        assert_eq!(writer.finish(), 1);
    }
    // Result byte: 101 1100 1 = 0b10111001 = 0xB9
    assert_eq!(buffer[0], 0xB9);

    let mut reader = BitReader::new(&buffer);
    assert_eq!(reader.read_bits(3).unwrap(), 0b101);
    assert_eq!(reader.read_bits(4).unwrap(), 0b1100);
    assert_eq!(reader.read_bits(1).unwrap(), 0b1);
}
