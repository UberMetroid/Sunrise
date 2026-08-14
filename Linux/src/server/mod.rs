// File: linux/src/server/mod.rs
// Title: Local BAP Emulation Server Module
// RFC Reference: RFC 793 (Transmission Control Protocol)
// Plain English: Listens on TCP, handles client sessions, and dispatches BAP opcodes.

pub mod session_handler;
pub mod tcp_server;

pub use session_handler::*;
pub use tcp_server::*;
