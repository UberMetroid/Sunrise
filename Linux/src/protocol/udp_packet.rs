// File: Linux/src/protocol/udp_packet.rs
// Title: UDP Game State Packet Codec
// RFC Reference: RFC 768 (User Datagram Protocol)
// Plain English: PlayerPosition, WorldSnapshot, and UDP opcode codec for combat sync.

use serde::{Deserialize, Serialize};
use crate::encoding::byte_order::*;
use crate::error::{Result, SunriseError};
use crate::protocol::udp_magic::UDP_MAGIC;

pub const UDP_HEADER_SIZE: usize = 12;
pub const UDP_MAX_PAYLOAD: usize = 64 * 1024;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl PlayerPosition {
    pub const WIRE_SIZE: usize = 20;

    pub fn encode(&self, target: &mut [u8]) -> Result<usize> {
        if target.len() < Self::WIRE_SIZE {
            return Err(SunriseError::BufferTooShort {
                needed: Self::WIRE_SIZE,
                available: target.len(),
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
                needed: Self::WIRE_SIZE,
                available: source.len(),
            });
        }
        let to_f32 = |s: &[u8]| f32::from_le_bytes(s.try_into().unwrap());
        Ok(Self {
            x: to_f32(&source[0..4]),
            y: to_f32(&source[4..8]),
            z: to_f32(&source[8..12]),
            yaw: to_f32(&source[12..16]),
            pitch: to_f32(&source[16..20]),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = vec![0u8; Self::WIRE_SIZE];
        let _ = self.encode(&mut out);
        out
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub sequence: u32,
    pub players: Vec<(u64, PlayerPosition)>,
}

impl WorldSnapshot {
    pub fn encode(&self, target: &mut [u8]) -> Result<usize> {
        let per = 8 + PlayerPosition::WIRE_SIZE;
        let needed = 4 + self.players.len() * per;
        if target.len() < needed {
            return Err(SunriseError::BufferTooShort { needed, available: target.len() });
        }
        write_u32_le(&mut target[..4], self.players.len() as u32);
        let mut off = 4;
        for (id, pos) in &self.players {
            target[off..off + 8].copy_from_slice(&id.to_le_bytes());
            off += 8;
            pos.encode(&mut target[off..])?;
            off += PlayerPosition::WIRE_SIZE;
        }
        Ok(needed)
    }

    pub fn decode(source: &[u8]) -> Result<Self> {
        if source.len() < 4 {
            return Err(SunriseError::BufferTooShort { needed: 4, available: source.len() });
        }
        let count = read_u32_le(&source[..4]).unwrap() as usize;
        let per = 8 + PlayerPosition::WIRE_SIZE;
        let needed = 4 + count * per;
        if source.len() < needed {
            return Err(SunriseError::BufferTooShort { needed, available: source.len() });
        }
        let mut players = Vec::with_capacity(count);
        let mut off = 4;
        for _ in 0..count {
            let mut id_bytes = [0u8; 8];
            id_bytes.copy_from_slice(&source[off..off + 8]);
            let id = u64::from_le_bytes(id_bytes);
            off += 8;
            let pos = PlayerPosition::decode(&source[off..])?;
            off += PlayerPosition::WIRE_SIZE;
            players.push((id, pos));
        }
        Ok(Self { sequence: 0, players })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpPacket {
    pub opcode: UdpOpcode,
    pub sequence: u32,
    pub payload: Vec<u8>,
}

impl UdpPacket {
    pub fn new(opcode: UdpOpcode, sequence: u32, payload: Vec<u8>) -> Self {
        Self { opcode, sequence, payload }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.encode().unwrap_or_default()
    }

    pub fn encoded_size(&self) -> usize {
        UDP_HEADER_SIZE + self.payload.len()
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.payload.len() > UDP_MAX_PAYLOAD {
            return Err(SunriseError::PayloadTooLarge {
                length: self.payload.len(),
                max: UDP_MAX_PAYLOAD,
            });
        }
        let mut buf = vec![0u8; self.encoded_size()];
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
                needed: UDP_HEADER_SIZE,
                available: buf.len(),
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
