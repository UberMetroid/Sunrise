// File: linux/src/server/mod.rs
// Title: Local BAP Emulation Server Module
// RFC Reference: RFC 793 (Transmission Control Protocol)
// Plain English: Listens on TCP, handles client sessions, and dispatches BAP opcodes.

pub mod session_handler;
pub mod tcp_server;
pub mod outbound_queue;
pub mod client_registry;
pub mod fireteam;
pub mod handlers;

pub use session_handler::*;
pub use tcp_server::*;
pub use outbound_queue::*;
pub use client_registry::*;
pub use fireteam::*;