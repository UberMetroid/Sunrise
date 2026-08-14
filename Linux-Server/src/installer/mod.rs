// File: Linux-Server/src/installer/mod.rs
// Title: Linux Installer Module
// Plain English: Coordinates Steam discovery, Destiny 2 detection, and .config setup.

pub mod steam_locator;
pub mod config_setup;
pub mod mod_installer;
pub mod desktop_entry;
pub mod ghost_narrative;
pub mod uninstaller;
pub mod doctor;
pub mod depot_downloader;

pub use steam_locator::*;
pub use config_setup::*;
pub use mod_installer::*;
pub use desktop_entry::*;
pub use ghost_narrative::*;
pub use uninstaller::*;
pub use doctor::*;
pub use depot_downloader::*;
