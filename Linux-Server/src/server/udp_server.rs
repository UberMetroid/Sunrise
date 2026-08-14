// File: Linux-Server/src/server/udp_server.rs
// Title: UDP Game State Listener & Snapshot Echo
// RFC Reference: RFC 768 (User Datagram Protocol)
// Plain English: Listens for UDP packets, updates per-player WorldState, echoes snapshots back.

use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::error::{Result, SunriseError};
use crate::protocol::udp_packet::{
    PlayerPosition, UdpOpcode, UdpPacket, WorldSnapshot, UDP_HEADER_SIZE,
};
use crate::server::client_registry::ClientRegistry;
use crate::server::world_state::WorldState;

pub struct SunriseUdpServer {
    bind_addr: SocketAddr,
    registry: Arc<ClientRegistry>,
    world: Arc<WorldState>,
    is_running: Arc<AtomicBool>,
}

impl SunriseUdpServer {
    pub fn new(
        bind_addr: SocketAddr,
        registry: Arc<ClientRegistry>,
        world: Arc<WorldState>,
        is_running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            bind_addr,
            registry,
            world,
            is_running,
        }
    }

    pub fn run(&self) -> Result<()> {
        let socket = UdpSocket::bind(self.bind_addr)
            .map_err(|e| SunriseError::IoError(format!("{}: {}", self.bind_addr, e)))?;
        socket.set_read_timeout(Some(Duration::from_millis(50)))?;
        println!("\x1b[1;32m[✓] UDP listener bound to {}\x1b[0m", self.bind_addr);

        let mut buf = vec![0u8; 4096];
        while self.is_running.load(Ordering::SeqCst) {
            match socket.recv_from(&mut buf) {
                Ok((n, peer)) => self.dispatch(&buf[..n], peer, &socket),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(_) => continue,
            }
        }
        Ok(())
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn build_snapshot_bytes(&self) -> Result<Vec<u8>> {
        let snap = self.world.snapshot();
        let payload = encode_snapshot_payload(&snap);
        let packet = UdpPacket::new(UdpOpcode::WorldSnapshot, snap.sequence, payload);
        packet.encode()
    }

    fn dispatch(&self, data: &[u8], peer: SocketAddr, socket: &UdpSocket) {
        if data.len() < UDP_HEADER_SIZE {
            return;
        }
        let packet = match UdpPacket::decode(data) {
            Ok(p) => p,
            Err(_) => return,
        };
        match packet.opcode {
            UdpOpcode::Heartbeat => {
                let resp = UdpPacket::new(UdpOpcode::Heartbeat, packet.sequence, Vec::new());
                if let Ok(bytes) = resp.encode() {
                    let _ = socket.send_to(&bytes, peer);
                }
            }
            UdpOpcode::PlayerPosition => {
                let pos = match decode_player_position(&packet.payload) {
                    Some(p) => p,
                    None => return,
                };
                if let Some(handle) = self.registry.lookup_by_udp(peer) {
                    self.world.update_position(handle.membership_id, pos);
                    if let Ok(bytes) = self.build_snapshot_bytes() {
                        let _ = socket.send_to(&bytes, peer);
                    }
                }
            }
            UdpOpcode::WorldSnapshot
            | UdpOpcode::BindAck
            | UdpOpcode::Unknown(_) => {}
        }
    }
}

fn decode_player_position(payload: &[u8]) -> Option<PlayerPosition> {
    if payload.len() < 20 {
        return None;
    }
    Some(PlayerPosition {
        x: f32::from_le_bytes(payload[0..4].try_into().ok()?),
        y: f32::from_le_bytes(payload[4..8].try_into().ok()?),
        z: f32::from_le_bytes(payload[8..12].try_into().ok()?),
        yaw: f32::from_le_bytes(payload[12..16].try_into().ok()?),
        pitch: f32::from_le_bytes(payload[16..20].try_into().ok()?),
    })
}

fn encode_snapshot_payload(snap: &WorldSnapshot) -> Vec<u8> {
    let per = 8 + PlayerPosition::WIRE_SIZE;
    let needed = 8 + snap.players.len() * per;
    let mut out = vec![0u8; needed];
    snap.encode(&mut out).expect("WorldSnapshot encode");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn bind_addr_round_trip() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
        let registry = ClientRegistry::new();
        let world = Arc::new(WorldState::new());
        let running = Arc::new(AtomicBool::new(false));
        let _server = SunriseUdpServer::new(addr, registry, world, running);
    }

    #[test]
    fn build_snapshot_bytes_returns_valid_frame() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
        let registry = ClientRegistry::new();
        let world = Arc::new(WorldState::new());
        let running = Arc::new(AtomicBool::new(false));
        world.update_position(1, PlayerPosition { x: 1.0, y: 2.0, z: 3.0, yaw: 0.0, pitch: 0.0 });
        world.update_position(2, PlayerPosition { x: 4.0, y: 5.0, z: 6.0, yaw: 1.0, pitch: 0.5 });
        let server = SunriseUdpServer::new(addr, registry, world, running);
        let bytes = server.build_snapshot_bytes().expect("snapshot encode");
        let pkt = UdpPacket::decode(&bytes).expect("snapshot decode");
        assert_eq!(pkt.opcode, UdpOpcode::WorldSnapshot);
        assert_eq!(pkt.payload.len(), 8 + 2 * (8 + 20));
    }
}
