// File: linux/src/protocol/mod.rs
// Title: Protocol Framing & Envelopes
// RFC Reference: RFC 793 (TCP Stream Framing), RFC 9110 (Web Service Framing)
// Plain English: Handles BAP framing, opcode translation, and service envelopes.

pub mod opcode;
pub mod bap_frame;
pub mod envelope;
pub mod udp_magic;
pub mod udp_packet;

pub use opcode::*;
pub use bap_frame::*;
pub use envelope::*;
pub use udp_magic::*;
pub use udp_packet::*;
