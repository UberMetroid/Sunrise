// File: linux-server/tests/test_udp_loopback.rs
// Title: UDP Transport & WorldState Loopback Tests
// RFC Reference: RFC 768 (User Datagram Protocol Loopback Tests)
// Plain English: Boots a UDP server, sends packets, asserts echo + WorldState updates.

use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use sunrise_linux::protocol::udp_packet::{
    PlayerPosition, UdpOpcode, UdpPacket, UDP_HEADER_SIZE,
};
use sunrise_linux::server::client_registry::ClientRegistry;
use sunrise_linux::server::udp_server::SunriseUdpServer;
use sunrise_linux::server::world_state::WorldState;

fn encode_position(pos: &PlayerPosition) -> Vec<u8> {
    let mut buf = vec![0u8; PlayerPosition::WIRE_SIZE];
    pos.encode(&mut buf).expect("encode PlayerPosition");
    buf
}

fn approx_eq(a: &PlayerPosition, b: &PlayerPosition) -> bool {
    let eps = 1e-4_f32;
    (a.x - b.x).abs() < eps
        && (a.y - b.y).abs() < eps
        && (a.z - b.z).abs() < eps
        && (a.yaw - b.yaw).abs() < eps
        && (a.pitch - b.pitch).abs() < eps
}

fn spawn_server(
    registry: Arc<ClientRegistry>,
    world: Arc<WorldState>,
    is_running: Arc<AtomicBool>,
) -> (SocketAddr, thread::JoinHandle<()>) {
    let probe = UdpSocket::bind("127.0.0.1:0").expect("probe bind");
    let server_addr = probe.local_addr().expect("probe local_addr");
    drop(probe);

    let server = SunriseUdpServer::new(server_addr, registry, world, is_running);
    let handle = thread::spawn(move || {
        let _ = server.run();
    });
    thread::sleep(Duration::from_millis(150));
    (server_addr, handle)
}

#[test]
fn test_udp_heartbeat_roundtrip() {
    let registry = ClientRegistry::new();
    let world = Arc::new(WorldState::new());
    let is_running = Arc::new(AtomicBool::new(true));
    let (server_addr, _server_thread) = spawn_server(registry, world, is_running.clone());

    let client = UdpSocket::bind("127.0.0.1:0").expect("client bind");
    client.set_read_timeout(Some(Duration::from_millis(1500))).unwrap();

    let pkt = UdpPacket::new(UdpOpcode::Heartbeat, 42, vec![]);
    let bytes = pkt.encode().expect("encode heartbeat");
    client.send_to(&bytes, server_addr).expect("send heartbeat");

    let mut buf = [0u8; 4096];
    let (n, src) = client.recv_from(&mut buf).expect("recv heartbeat echo");
    assert_eq!(src, server_addr);
    let resp = UdpPacket::decode(&buf[..n]).expect("decode echo");
    assert_eq!(resp.opcode, UdpOpcode::Heartbeat);
    assert_eq!(resp.sequence, 42);
    assert_eq!(n, UDP_HEADER_SIZE);

    is_running.store(false, Ordering::Relaxed);
}

#[test]
fn test_player_position_updates_world_state() {
    let client = UdpSocket::bind("127.0.0.1:0").expect("client bind");
    let client_addr = client.local_addr().unwrap();
    client.set_read_timeout(Some(Duration::from_millis(1500))).unwrap();

    let registry = ClientRegistry::new();
    registry.register(1001, "TestPlayer", client_addr);

    let world = Arc::new(WorldState::new());
    let is_running = Arc::new(AtomicBool::new(true));
    let (server_addr, _server_thread) =
        spawn_server(registry, world.clone(), is_running.clone());

    let pos = PlayerPosition { x: 1.0, y: 2.0, z: 3.0, yaw: 0.5, pitch: -0.25 };
    let pkt = UdpPacket::new(UdpOpcode::PlayerPosition, 1, encode_position(&pos));
    let bytes = pkt.encode().expect("encode PlayerPosition packet");
    client.send_to(&bytes, server_addr).expect("send PlayerPosition");

    let mut buf = [0u8; 4096];
    let (n, src) = client.recv_from(&mut buf).expect("recv snapshot echo");
    assert_eq!(src, server_addr);
    let resp = UdpPacket::decode(&buf[..n]).expect("decode WorldSnapshot");
    assert_eq!(resp.opcode, UdpOpcode::WorldSnapshot);

    let snap = world.snapshot();
    let found = snap
        .players
        .iter()
        .any(|(id, p)| *id == 1001 && approx_eq(p, &pos));
    assert!(
        found,
        "WorldState snapshot must contain membership 1001 with our position"
    );

    is_running.store(false, Ordering::Relaxed);
}

#[test]
fn test_world_snapshot_echo_contains_other_players() {
    let client_a = UdpSocket::bind("127.0.0.1:54321").expect("bind A");
    let client_b = UdpSocket::bind("127.0.0.1:54331").expect("bind B");
    let addr_a = client_a.local_addr().unwrap();
    let addr_b = client_b.local_addr().unwrap();
    client_a.set_read_timeout(Some(Duration::from_millis(1500))).unwrap();
    client_b.set_read_timeout(Some(Duration::from_millis(1500))).unwrap();

    let registry = ClientRegistry::new();
    registry.register(1001, "Alice", addr_a);
    registry.register(1002, "Bob", addr_b);

    let world = Arc::new(WorldState::new());
    let is_running = Arc::new(AtomicBool::new(true));
    let (server_addr, _server_thread) =
        spawn_server(registry, world.clone(), is_running.clone());

    let pos_a = PlayerPosition { x: 10.0, y: 20.0, z: 30.0, yaw: 0.0, pitch: 0.0 };
    let pos_b = PlayerPosition { x: -5.0, y: 0.0, z: 7.5, yaw: 1.5, pitch: -0.5 };

    let pkt_a = UdpPacket::new(UdpOpcode::PlayerPosition, 1, encode_position(&pos_a));
    client_a
        .send_to(&pkt_a.encode().unwrap(), server_addr)
        .expect("send A");
    thread::sleep(Duration::from_millis(80));

    let pkt_b = UdpPacket::new(UdpOpcode::PlayerPosition, 2, encode_position(&pos_b));
    client_b
        .send_to(&pkt_b.encode().unwrap(), server_addr)
        .expect("send B");
    thread::sleep(Duration::from_millis(80));

    let probe = UdpSocket::bind("127.0.0.1:54341").expect("bind probe");
    probe.set_read_timeout(Some(Duration::from_millis(1500))).unwrap();
    let hb = UdpPacket::new(UdpOpcode::Heartbeat, 99, vec![]);
    probe
        .send_to(&hb.encode().unwrap(), server_addr)
        .expect("send heartbeat");
    let mut buf = [0u8; 4096];
    let (n, _) = probe.recv_from(&mut buf).expect("recv heartbeat echo");
    let resp = UdpPacket::decode(&buf[..n]).expect("decode");
    assert_eq!(resp.opcode, UdpOpcode::Heartbeat);
    assert_eq!(resp.sequence, 99);

    let snap = world.snapshot();
    let by_id: std::collections::HashMap<u64, PlayerPosition> =
        snap.players.into_iter().collect();
    assert_eq!(by_id.len(), 2, "snapshot must have both registered players");
    assert!(approx_eq(by_id.get(&1001).expect("Alice present"), &pos_a));
    assert!(approx_eq(by_id.get(&1002).expect("Bob present"), &pos_b));

    is_running.store(false, Ordering::Relaxed);
}