// File: linux/src/lib.rs
// Title: Sunrise Linux Runtime Library
// RFC References:
//   - RFC 793 (TCP Protocol)
//   - RFC 9110 (HTTP / Web Services Semantics)
//   - RFC 5116 (AEAD Cryptography)
//   - RFC 6234 (SHA-256 Hashing)
//   - RFC 2104 (HMAC)
//   - RFC 8259 (JSON Interchange Format)
// Plain English: Project Sunrise native Linux runtime, emulation backend, and test suite.

pub mod error;
pub mod encoding;
pub mod crypto;
pub mod protocol;
pub mod state;
pub mod settings;
pub mod server;
pub mod installer;
pub mod tui;

pub use error::{Result, SunriseError};
pub use encoding::*;
pub use crypto::*;
pub use protocol::*;
pub use state::*;
pub use settings::*;
pub use server::*;
pub use installer::*;
pub use tui::*;

pub const SUNRISE_LINUX_VERSION: &str = "0.4.8";
