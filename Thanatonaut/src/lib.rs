// File: Thanatonaut/src/lib.rs
// Title: Thanatonaut Runtime Library
// Plain English: Private standalone Destiny 2 server emulator and world simulation engine.

pub mod error;
pub mod encoding;
pub mod crypto;
pub mod protocol;
pub mod state;
pub mod manifest;
pub mod settings;
pub mod server;
pub mod installer;
pub mod tui;

pub use error::{Result, SunriseError};
pub use encoding::*;
pub use crypto::*;
pub use protocol::*;
pub use state::*;
pub use manifest::*;
pub use settings::*;
pub use server::*;
pub use installer::*;
pub use tui::*;

pub const THANATONAUT_VERSION: &str = "0.6.4";
