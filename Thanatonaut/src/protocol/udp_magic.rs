// File: Thanatonaut/src/protocol/udp_magic.rs
// Title: UDP Frame Magic Constant
// RFC Reference: RFC 768 (User Datagram Protocol)
// Plain English: "SUNU" 4-byte magic prefix identifying a Sunrise UDP packet.

pub const UDP_MAGIC: [u8; 4] = [0x53, 0x55, 0x4E, 0x55]; // "SUNU"

pub fn is_valid_magic(buf: &[u8]) -> bool {
    buf.len() >= 4 && buf[..4] == UDP_MAGIC
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_matches_ascii() {
        assert_eq!(&UDP_MAGIC, b"SUNU");
    }

    #[test]
    fn rejects_short_buffer() {
        assert!(!is_valid_magic(&[]));
        assert!(!is_valid_magic(&[0x53, 0x55]));
    }

    #[test]
    fn accepts_correct_prefix() {
        let mut buf = vec![0x53, 0x55, 0x4E, 0x55];
        buf.extend_from_slice(&[0x01, 0x02, 0x03]);
        assert!(is_valid_magic(&buf));
    }

    #[test]
    fn rejects_wrong_prefix() {
        let buf = [0xDE, 0xAD, 0xBE, 0xEF, 0x00];
        assert!(!is_valid_magic(&buf));
    }
}