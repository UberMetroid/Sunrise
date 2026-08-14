// File: linux/src/settings/config.rs
// Title: JSON Settings Parser & Validator
// RFC Reference: RFC 8259 (JSON Data Interchange Format)
// Plain English: Reads, writes, and validates runtime settings from a JSON file or string.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::error::{Result, SunriseError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub enable_queuez: bool,
    pub max_connections: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 7777,
            enable_queuez: true,
            max_connections: 64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SunriseSettings {
    pub version: String,
    pub server: ServerConfig,
    pub auto_unlock_entitlements: bool,
    pub default_power_cap: u32,
}

impl Default for SunriseSettings {
    fn default() -> Self {
        Self {
            version: "0.4.2".to_string(),
            server: ServerConfig::default(),
            auto_unlock_entitlements: true,
            default_power_cap: 1000,
        }
    }
}

impl SunriseSettings {
    pub fn from_json_str(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str).map_err(|e| SunriseError::InvalidJson(e.to_string()))
    }

    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| SunriseError::InvalidJson(e.to_string()))
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Self> {
        let p = path.as_ref();
        if !p.exists() {
            return Err(SunriseError::FileNotFound(p.to_string_lossy().to_string()));
        }
        let content = fs::read_to_string(p)
            .map_err(|e| SunriseError::IoError(e.to_string()))?;
        Self::from_json_str(&content)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = self.to_json_string()?;
        fs::write(path, json).map_err(|e| SunriseError::IoError(e.to_string()))?;
        Ok(())
    }
}
