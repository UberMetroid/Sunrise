// File: Linux-Server/src/protocol/udp_packet.rs
// Title: UDP Game State Packet Codec
// RFC Reference: RFC 768 (User Datagram Protocol)
// Plain English: PlayerPosition, WorldSnapshot, and UDP opcode codec for combat/physics sync.

use crate::encoding::byte_order::*;
use crate::error::{Result, SunriseError};
use crate::protocol::udp_magic::UDP_MAGIC;

pub const UDP_HEADER_SIZE: usize = 12;
pub const UDP_MAX_PAYLOAD: usize = 64 * 1024;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdpOpcode {
    Heartbeat = 1,
    PlayerPosition = 2,
    WorldSnapshot = 3,
    BindAck = 4,
    Unknown(u8),
}

impl From<u8> for UdpOpcode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Heartbeat,
            2 => Self::PlayerPosition,
            3 => Self::WorldSnapshot,
            4 => Self::BindAck,
            other => Self::Unknown(other),
        }
    }
}

impl From<UdpOpcode> for u8 {
    fn from(op: UdpOpcode) -> Self {
        match op {
            UdpOpcode::Heartbeat => 1,
            UdpOpcode::PlayerPosition => 2,
            UdpOpcode::WorldSnapshot => 3,
            UdpOpcode::BindAck => 4,
            UdpOpcode::Unknown(v) => v,
        }
    }
}

impl std::fmt::Display for UdpOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Heartbeat => write!(f, "Heartbeat (0x01)"), Self::PlayerPosition => write!(f, "PlayerPosition (0x02)"), Self::WorldSnapshot => write!(f, "WorldSnapshot (0x03)"), Self::BindAck => write!(f, "BindAck (0x04)"), Self::Unknown(v) => write!(f, "UnknownUdp (0x{:02X})", v) }
    }
}#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerPosition { pub x: f32, pub y: f32, pub z: f32, pub yaw: f32, pub pitch: f32 }
impl PlayerPosition {
    pub const WIRE_SIZE: usize = 20;
    pub fn encode(&self, target: &mut [u8]) -> Result<usize> {
        if target.len() < Self::WIRE_SIZE {
            return Err(SunriseError::BufferTooShort {
                needed: Self::WIRE_SIZE, available: target.len(),
            });
        }
        target[..4].copy_from_slice(&self.x.to_le_bytes());
        target[4..8].copy_from_slice(&self.y.to_le_bytes());
        target[8..12].copy_from_slice(&self.z.to_le_bytes());
        target[12..16].copy_from_slice(&self.yaw.to_le_bytes());
        target[16..20].copy_from_slice(&self.pitch.to_le_bytes());
        Ok(Self::WIRE_SIZE)
    }
    pub fn decode(source: &[u8]) -> Result<Self> {
        if source.len() < Self::WIRE_SIZE {
            return Err(SunriseError::BufferTooShort {
                needed: Self::WIRE_SIZE, available: source.len(),
            });
        }
        Ok(Self {
            x: f32::from_le_bytes(source[0..4].try_into().unwrap()),
            y: f32::from_le_bytes(source[4..8].try_into().unwrap()),
            z: f32::from_le_bytes(source[8..12].try_into().unwrap()),
            yaw: f32::from_le_bytes(source[12..16].try_into().unwrap()),
            pitch: f32::from_le_bytes(source[16..20].try_into().unwrap()),
        })
    }
}#[derive(Debug, Clone, PartialEq)]
pub struct WorldSnapshot { pub sequence: u32, pub players: Vec<(u64, PlayerPosition)> }
impl WorldSnapshot {
    pub fn encode(&self, target: &mut [u8]) -> Result<usize> {
        let per = 8 + PlayerPosition::WIRE_SIZE;
        let max_players = UDP_MAX_PAYLOAD / per;
        if self.players.len() > max_players {
            return Err(SunriseError::PayloadTooLarge {
                length: self.players.len(),
                max: max_players,
            });
        }
        let needed = 8 + self.players.len() * per;
        if target.len() < needed {
            return Err(SunriseError::BufferTooShort { needed, available: target.len() });
        }
        write_u32_le(&mut target[..4], self.sequence);
        write_u32_le(&mut target[4..8], self.players.len() as u32);
        let mut off = 8;
        for (id, pos) in &self.players {
            target[off..off + 8].copy_from_slice(&id.to_le_bytes());
            off += 8;
            pos.encode(&mut target[off..])?;
            off += PlayerPosition::WIRE_SIZE;
        }
        Ok(needed)
    }
    pub fn decode(source: &[u8]) -> Result<Self> {
        if source.len() < 8 {
            return Err(SunriseError::BufferTooShort { needed: 8, available: source.len() });
        }
        let sequence = read_u32_le(&source[..4]).unwrap();
        let count = read_u32_le(&source[4..8]).unwrap() as usize;
        let per = 8 + PlayerPosition::WIRE_SIZE;
        let max_players = UDP_MAX_PAYLOAD / per;
        if count > max_players {
            return Err(SunriseError::PayloadTooLarge { length: count, max: max_players });
        }
        let needed = 8 + count * per;
        if source.len() < needed {
            return Err(SunriseError::BufferTooShort { needed, available: source.len() });
        }
        let mut players = Vec::with_capacity(count);
        let mut off = 8;
        for _ in 0..count {
            let mut id_bytes = [0u8; 8];
            id_bytes.copy_from_slice(&source[off..off + 8]);
            let id = u64::from_le_bytes(id_bytes);
            off += 8;
            let pos = PlayerPosition::decode(&source[off..])?;
            off += PlayerPosition::WIRE_SIZE;
            players.push((id, pos));
        }
        Ok(Self { sequence, players })
    }
}#[derive(Debug, Clone)]
pub struct UdpPacket { pub opcode: UdpOpcode, pub sequence: u32, pub payload: Vec<u8> }
impl UdpPacket { pub fn new(opcode: UdpOpcode, sequence: u32, payload: Vec<u8>) -> Self { Self { opcode, sequence, payload } }
    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.payload.len() > UDP_MAX_PAYLOAD {
            return Err(SunriseError::PayloadTooLarge {
                length: self.payload.len(), max: UDP_MAX_PAYLOAD,
            });
        }
        let mut buf = vec![0u8; UDP_HEADER_SIZE + self.payload.len()];
        buf[..4].copy_from_slice(&UDP_MAGIC);
        buf[4] = u8::from(self.opcode);
        buf[5] = 0;
        buf[6] = 0;
        buf[7] = 0;
        write_u32_be(&mut buf[8..12], self.sequence);
        buf[12..].copy_from_slice(&self.payload);
        Ok(buf)
    }
    pub fn decode(buf: &[u8]) -> Result<Self> {
        if buf.len() < UDP_HEADER_SIZE {
            return Err(SunriseError::BufferTooShort {
                needed: UDP_HEADER_SIZE, available: buf.len(),
            });
        }
        if buf[..4] != UDP_MAGIC {
            let mut got = [0u8; 4];
            got.copy_from_slice(&buf[..4]);
            return Err(SunriseError::InvalidMagicNumber(got));
        }
        let opcode = UdpOpcode::from(buf[4]);
        let sequence = read_u32_be(&buf[8..12]).unwrap();
        let payload = buf[12..].to_vec();
        Ok(Self { opcode, sequence, payload })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pos(x: f32, y: f32, z: f32) -> PlayerPosition {
        PlayerPosition { x, y, z, yaw: 0.0, pitch: 0.0 }
    }
    #[test]
    fn magic_roundtrip() {
        let pkt = UdpPacket {
            opcode: UdpOpcode::Heartbeat,
            sequence: 0xDEADBEEF,
            payload: Vec::new(),
        };
        let encoded = pkt.encode().expect("encode");
        assert_eq!(encoded.len(), UDP_HEADER_SIZE);
        assert_eq!(&encoded[..4], &UDP_MAGIC);
        let decoded = UdpPacket::decode(&encoded).expect("decode");
        assert_eq!(decoded.opcode, pkt.opcode);
        assert_eq!(decoded.sequence, pkt.sequence);
        assert_eq!(decoded.payload, pkt.payload);
    }
    #[test]
    fn player_position_payload_size() {
        let p = PlayerPosition { x: 1.0, y: 2.0, z: 3.0, yaw: 0.5, pitch: -0.25 };
        let mut buf = vec![0u8; PlayerPosition::WIRE_SIZE];
        let written = p.encode(&mut buf).expect("encode");
        assert_eq!(written, 20);
        let decoded = PlayerPosition::decode(&buf).expect("decode");
        assert_eq!(decoded, p);
    }
    #[test]
    fn world_snapshot_encode_decode() {
        let snap = WorldSnapshot {
            sequence: 99,
            players: vec![
                (0x0102030405060708, pos(1.0, 2.0, 3.0)),
                (0x1112131415161718, pos(-4.0, 5.0, 6.0)),
                (0xDEADBEEFCAFEBABE, pos(7.5, -8.25, 9.125)),
            ],
        };
        let mut buf = vec![0u8; 4096];
        let written = snap.encode(&mut buf).expect("encode");
        assert_eq!(written, 8 + 3 * (8 + 20));
        let seq = read_u32_le(&buf[..4]).unwrap();
        assert_eq!(seq, 99);
        let count = read_u32_le(&buf[4..8]).unwrap();
        assert_eq!(count, 3);
        let id0 = u64::from_le_bytes(buf[8..16].try_into().unwrap());
        assert_eq!(id0, 0x0102030405060708);
        let x0 = f32::from_le_bytes(buf[16..20].try_into().unwrap());
        assert_eq!(x0, 1.0);
        let id1 = u64::from_le_bytes(buf[36..44].try_into().unwrap());
        assert_eq!(id1, 0x1112131415161718);
        let decoded = WorldSnapshot::decode(&buf[..written]).expect("decode");
        assert_eq!(decoded.sequence, snap.sequence);
        assert_eq!(decoded.players, snap.players);
    }
    #[test]
    fn decode_rejects_short_buffer() {
        let result = UdpPacket::decode(&[0u8; 11]);
        assert!(matches!(result, Err(SunriseError::BufferTooShort { .. })));
    }
    #[test]
    fn decode_rejects_bad_magic() {
        let mut buf = vec![0u8; UDP_HEADER_SIZE];
        buf[..4].copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
        let result = UdpPacket::decode(&buf);
        assert!(matches!(result, Err(SunriseError::InvalidMagicNumber(_))));
    }
    #[test]
    fn decode_maps_unknown_opcode() {
        let mut buf = vec![0u8; UDP_HEADER_SIZE];
        buf[..4].copy_from_slice(&UDP_MAGIC);
        buf[4] = 0xFF;
        let pkt = UdpPacket::decode(&buf).expect("decode");
        assert_eq!(pkt.opcode, UdpOpcode::Unknown(0xFF));
    }
}
