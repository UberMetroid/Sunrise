// File: linux/src/server/tcp_server.rs
// Title: TCP BAP Server with Multi-Client Registry & Fireteam Routing
// RFC Reference: RFC 793 (Transmission Control Protocol)
// Plain English: Listens on a TCP port, registers clients in a shared registry, drains per-connection outbound frames between reads.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::error::{Result, SunriseError};
use crate::protocol::bap_frame::BapFrame;
use crate::server::client_registry::ClientRegistry;
use crate::server::fireteam::Fireteam;
use crate::server::session_handler::SessionHandler;
use crate::settings::config::ServerConfig;

pub struct SunriseTcpServer {
    config: ServerConfig,
    is_running: Arc<AtomicBool>,
}

impl SunriseTcpServer {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }

    pub fn run(&self) -> Result<()> {
        let bind_addr = format!("{}:{}", self.config.bind_address, self.config.port);
        let listener = TcpListener::bind(&bind_addr)
            .map_err(|_| SunriseError::AddressInUse(bind_addr.clone()))?;

        listener
            .set_nonblocking(true)
            .map_err(|e| SunriseError::IoError(e.to_string()))?;

        let registry = ClientRegistry::new();
        let fireteam = Arc::new(Fireteam::new(registry.clone()));
        let next_ephemeral = Arc::new(AtomicU64::new(1000));

        self.is_running.store(true, Ordering::SeqCst);
        println!("\x1b[1;32m[✓] BAP Emulation Server listening on {}\x1b[0m", bind_addr);
        println!("\x1b[1;33m[*] Waiting for Guardian connections from game client...\x1b[0m\n");

        while self.is_running.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, peer_addr)) => {
                    let running_flag = Arc::clone(&self.is_running);
                    let registry_clone = registry.clone();
                    let fireteam_clone = fireteam.clone();
                    let next_ephemeral_clone = next_ephemeral.clone();
                    println!("\x1b[1;36m[+] Guardian connected from {}\x1b[0m", peer_addr);
                    thread::spawn(move || {
                        let _ = Self::handle_connection(
                            stream,
                            peer_addr,
                            running_flag,
                            registry_clone,
                            fireteam_clone,
                            next_ephemeral_clone,
                        );
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    return Err(SunriseError::IoError(e.to_string()));
                }
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_connection(
        mut stream: TcpStream,
        peer_addr: SocketAddr,
        is_running: Arc<AtomicBool>,
        registry: Arc<ClientRegistry>,
        fireteam: Arc<Fireteam>,
        next_ephemeral: Arc<AtomicU64>,
    ) -> Result<()> {
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;

        let ephemeral_id = next_ephemeral.fetch_add(1, Ordering::Relaxed);
        let display_name = format!("Ephemeral-{}", ephemeral_id);
        let mut session = SessionHandler::new(
            ephemeral_id,
            display_name,
            peer_addr,
            registry.clone(),
            fireteam.clone(),
        );

        let mut buffer = vec![0u8; 65536];

        while is_running.load(Ordering::SeqCst) {
            // 1) Drain any pushed frames (fanout from peers) before blocking on read.
            while let Some(pushed) = session.registry_handle.outbound.try_drain_next() {
                let bytes = pushed.to_bytes()?;
                if stream.write_all(&bytes).is_err() {
                    break;
                }
                let _ = stream.flush();
            }

            let bytes_read = match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                    continue;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(_) => break,
            };

            let mut cursor = 0;
            while cursor < bytes_read {
                match BapFrame::decode(&buffer[cursor..bytes_read]) {
                    Ok((frame, consumed)) => {
                        cursor += consumed;
                        println!(
                            "  \x1b[38;5;51m✦\x1b[0m \x1b[1m[Session {}]\x1b[0m Handled Opcode: \x1b[1;33m{:?}\x1b[0m (Tx: {}, Payload: {} bytes)",
                            session.account.membership_id,
                            frame.opcode,
                            frame.transaction_id,
                            frame.payload.len()
                        );
                        let response_frame = session.handle_frame(&frame)?;
                        let response_bytes = response_frame.to_bytes()?;
                        stream.write_all(&response_bytes)?;
                        stream.flush()?;
                    }
                    Err(_) => break,
                }
            }
        }

        registry.unregister(session.account.membership_id);
        fireteam.leave(session.active_destination_hash, session.account.membership_id);

        println!("\x1b[1;31m[-] Guardian disconnected ({})\x1b[0m", peer_addr);
        Ok(())
    }
}