// File: Linux-Server/src/settings/config.rs
// Title: JSON Settings Parser & Validator
// RFC Reference: RFC 8259 (JSON Data Interchange Format)
// Plain English: Reads, writes, and validates runtime settings from a JSON file or string.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::get_home_dir;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub udp_bind_address: String,
    pub udp_port: u16,
    pub enable_queuez: bool,
    pub max_connections: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 7777,
            udp_bind_address: "127.0.0.1".to_string(),
            udp_port: 7778,
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
            version: "0.6.5".to_string(),
            server: ServerConfig::default(),
            auto_unlock_entitlements: true,
            default_power_cap: 1000,
        }
    }
}

fn allowed_bases() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/data"),
        get_home_dir().join(".config").join("sunrise"),
    ]
}

fn is_within_allowed(path: &Path) -> bool {
    if path.starts_with("/tmp") {
        return true;
    }
    for base in allowed_bases() {
        if path.starts_with(&base) {
            return true;
        }
        if let Ok(canon_base) = fs::canonicalize(&base) {
            if let Ok(canon_path) = fs::canonicalize(path) {
                if canon_path.starts_with(&canon_base) {
                    return true;
                }
            }
            if let Some(parent) = path.parent() {
                if let Ok(canon_parent) = fs::canonicalize(parent) {
                    if canon_parent.starts_with(&canon_base) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn validate_path(path: &Path) -> Result<()> {
    for comp in path.components() {
        if matches!(comp, std::path::Component::ParentDir) {
            return Err(SunriseError::IoError(format!(
                "path traversal rejected: {}",
                path.display()
            )));
        }
    }
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.file_type().is_symlink() {
            return Err(SunriseError::IoError(format!(
                "symlink traversal rejected: {}",
                path.display()
            )));
        }
    }
    if let Some(parent) = path.parent() {
        if let Ok(meta) = fs::symlink_metadata(parent) {
            if meta.file_type().is_symlink() {
                return Err(SunriseError::IoError(format!(
                    "symlink traversal rejected: {}",
                    path.display()
                )));
            }
        }
        if !is_within_allowed(parent) && !is_within_allowed(path) {
            return Err(SunriseError::IoError(format!(
                "path outside allowed base: {}",
                path.display()
            )));
        }
        if parent.exists() {
            if let Ok(canon_parent) = fs::canonicalize(parent) {
                let mut allowed = false;
                for base in allowed_bases() {
                    if let Ok(canon_base) = fs::canonicalize(&base) {
                        if canon_parent.starts_with(&canon_base) {
                            allowed = true;
                            break;
                        }
                    } else if canon_parent.starts_with(&base) {
                        allowed = true;
                        break;
                    }
                }
                if canon_parent.starts_with("/tmp") {
                    allowed = true;
                }
                if !allowed {
                    return Err(SunriseError::IoError(format!(
                        "canonical path outside allowed base: {}",
                        path.display()
                    )));
                }
            }
        }
    }
    Ok(())
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
        validate_path(p)?;
        if !p.exists() {
            return Err(SunriseError::FileNotFound(p.to_string_lossy().to_string()));
        }
        let content = fs::read_to_string(p)
            .map_err(|e| SunriseError::IoError(e.to_string()))?;
        Self::from_json_str(&content)
    }

    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let p = path.as_ref();
        validate_path(p)?;
        if let Some(parent) = p.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = self.to_json_string()?;
        fs::write(p, json).map_err(|e| SunriseError::IoError(e.to_string()))?;
        Ok(())
    }
}
