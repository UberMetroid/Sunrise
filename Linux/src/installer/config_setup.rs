// File: linux/src/installer/config_setup.rs
// Title: XDG Base Directory Configuration Setup
// RFC Reference: RFC 8259 (JSON Interchange Format)
// Plain English: Initializes the ~/.config/sunrise/ directory structure and default config file.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::get_home_dir;
use crate::settings::config::SunriseSettings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SunriseDirectories {
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
    pub cache_dir: PathBuf,
    pub profiles_dir: PathBuf,
}

impl SunriseDirectories {
    pub fn default_paths() -> Self {
        let home = get_home_dir();
        let config_dir = home.join(".config").join("sunrise");
        let config_file = config_dir.join("config.json");
        let cache_dir = config_dir.join("cache");
        let profiles_dir = config_dir.join("profiles");

        Self {
            config_dir,
            config_file,
            cache_dir,
            profiles_dir,
        }
    }

    pub fn initialize(&self, game_root: Option<&Path>) -> Result<()> {
        // Create directory tree
        fs::create_dir_all(&self.config_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create config dir: {}", e)))?;
        fs::create_dir_all(&self.cache_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create cache dir: {}", e)))?;
        fs::create_dir_all(&self.profiles_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create profiles dir: {}", e)))?;

        // Write default configuration if it does not already exist
        if !self.config_file.exists() {
            let settings = SunriseSettings::default();
            settings.save_to_file(&self.config_file)?;
        }

        // Write game root path marker if detected
        if let Some(root) = game_root {
            let marker_file = self.config_dir.join("game_path.txt");
            let _ = fs::write(marker_file, root.to_string_lossy().as_bytes());
        }

        Ok(())
    }
}
