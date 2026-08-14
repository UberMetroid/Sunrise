// File: Linux/tests/test_udp_loopback.rs
// Title: UDP Transport & WorldState Loopback Tests
// RFC Reference: RFC 768 (User Datagram Protocol Loopback Tests)
// Plain English: Boots a UDP server, sends packets, asserts echo + WorldState updates.

use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use sunrise_linux::protocol::udp_packet::{PlayerPosition, UdpOpcode, UdpPacket};
use sunrise_linux::server::client_registry::ClientRegistry;
use sunrise_linux::server::udp_server::SunriseUdpServer;
use sunrise_linux::server::world_state::WorldState;

fn spawn_server(
    registry: Arc<ClientRegistry>,
    world: Arc<WorldState>,
    is_running: Arc<AtomicBool>,
) -> (SocketAddr, thread::JoinHandle<()>) {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("server bind");
    let server_addr = socket.local_addr().expect("local_addr");
    drop(socket);

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
    client.send_to(&pkt.to_bytes(), server_addr).expect("send heartbeat");

    let mut buf = [0u8; 4096];
    let (n, src) = client.recv_from(&mut buf).expect("recv heartbeat echo");
    assert_eq!(src, server_addr);
    let resp = UdpPacket::decode(&buf[..n]).expect("decode echo");
    assert_eq!(resp.opcode, UdpOpcode::Heartbeat);
    assert_eq!(resp.sequence, 42);

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
    let (server_addr, _server_thread) = spawn_server(registry, world.clone(), is_running.clone());

    let pos = PlayerPosition { x: 1.0, y: 2.0, z: 3.0, yaw: 0.0, pitch: 0.0 };
    let pkt = UdpPacket::new(UdpOpcode::PlayerPosition, 1, pos.to_bytes());
    client.send_to(&pkt.to_bytes(), server_addr).unwrap();

    let mut buf = [0u8; 4096];
    let (n, src) = client.recv_from(&mut buf).expect("recv snapshot echo");
    assert_eq!(src, server_addr);
    let resp = UdpPacket::decode(&buf[..n]).unwrap();
    assert_eq!(resp.opcode, UdpOpcode::WorldSnapshot);

    let snap = world.snapshot();
    let found = snap.players.iter().any(|(id, p)| *id == 1001 && approx_eq(p, &pos));
    assert!(found, "snapshot must contain membership 1001 with our position");

    is_running.store(false, Ordering::Relaxed);
}

fn approx_eq(a: &PlayerPosition, b: &PlayerPosition) -> bool {
    let eps = 1e-4_f32;
    (a.x - b.x).abs() < eps
        && (a.y - b.y).abs() < eps
        && (a.z - b.z).abs() < eps
        && (a.yaw - b.yaw).abs() < eps
        && (a.pitch - b.pitch).abs() < eps
}