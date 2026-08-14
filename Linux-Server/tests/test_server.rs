// File: linux/tests/test_server.rs
// Title: TCP BAP Server Loopback E-M Verification Proofs
// RFC Reference: RFC 793 (Transmission Control Protocol)
// Plain English: Boots a local TCP server, establishes a client stream, sends a BAP frame, and asserts response.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use sunrise_linux::protocol::bap_frame::BapFrame;
use sunrise_linux::protocol::opcode::Opcode;
use sunrise_linux::server::tcp_server::SunriseTcpServer;
use sunrise_linux::settings::config::ServerConfig;

#[test]
fn test_tcp_server_loopback_roundtrip() {
    let port = 17890;
    let mut config = ServerConfig::default();
    config.bind_address = "127.0.0.1".to_string();
    config.port = port;

    let server = SunriseTcpServer::new(config);

    // Spawn server in background thread
    let _server_thread = thread::spawn(move || {
        let _ = server.run();
    });

    // Give server time to bind
    thread::sleep(Duration::from_millis(50));

    // Connect client
    let mut client = TcpStream::connect(format!("127.0.0.1:{}", port))
        .expect("Client should connect to TCP server");

    client.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

    // Send QueueZ frame
    let request_frame = BapFrame::new(999, Opcode::QueueZ, vec![0x01, 0x02]);
    let req_bytes = request_frame.to_bytes().unwrap();
    client.write_all(&req_bytes).unwrap();
    client.flush().unwrap();

    // Read response
    let mut resp_buf = vec![0u8; 1024];
    let n = client.read(&mut resp_buf).expect("Should read response");
    assert!(n > 0);

    let (response_frame, consumed) = BapFrame::decode(&resp_buf[..n])
        .expect("Response should decode as BapFrame");

    assert_eq!(consumed, n);
    assert_eq!(response_frame.transaction_id, 999);
    assert_eq!(response_frame.opcode, Opcode::QueueZ);
    assert_eq!(response_frame.payload, vec![0x08, 0x01, 0x10, 0x00]);
}
