// File: linux/src/server/tcp_server.rs
// Title: TCP BAP Server
// RFC Reference: RFC 793 (Transmission Control Protocol)
// Plain English: Listens on a TCP port and executes request/response loops with game clients.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::error::{Result, SunriseError};
use crate::protocol::bap_frame::BapFrame;
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

        self.is_running.store(true, Ordering::SeqCst);

        while self.is_running.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let running_flag = Arc::clone(&self.is_running);
                    thread::spawn(move || {
                        let _ = Self::handle_connection(stream, running_flag);
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

    fn handle_connection(mut stream: TcpStream, is_running: Arc<AtomicBool>) -> Result<()> {
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;

        let mut session = SessionHandler::new(1001, "Guardian");
        let mut buffer = vec![0u8; 65536];

        while is_running.load(Ordering::SeqCst) {
            let bytes_read = match stream.read(&mut buffer) {
                Ok(0) => break, // EOF / connection closed
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
                        let response_frame = session.handle_frame(&frame)?;
                        let response_bytes = response_frame.to_bytes()?;
                        stream.write_all(&response_bytes)?;
                        stream.flush()?;
                    }
                    Err(_) => break,
                }
            }
        }

        Ok(())
    }
}
