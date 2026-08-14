// File: Linux-Server/src/manifest/manifest_store.rs
// Title: World Content Manifest Database Store
// Plain English: In-memory hash lookup cache with file persistence for fast weapon and armor queries.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::get_home_dir;
use crate::manifest::item_definition::DestinyItemDefinition;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ManifestStore {
    pub items: HashMap<u32, DestinyItemDefinition>,
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

impl ManifestStore {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn default_cache_path() -> PathBuf {
        let path = if Path::new("/data").exists() {
            PathBuf::from("/data/manifest_cache.json")
        } else {
            get_home_dir().join(".config").join("sunrise").join("manifest_cache.json")
        };
        path
    }

    pub fn insert_item(&mut self, item: DestinyItemDefinition) {
        self.items.insert(item.item_hash, item);
    }

    pub fn get_item(&self, hash: u32) -> Option<&DestinyItemDefinition> {
        self.items.get(&hash)
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&DestinyItemDefinition> {
        let q = query.to_lowercase();
        self.items
            .values()
            .filter(|item| item.name.to_lowercase().contains(&q))
            .collect()
    }

    pub fn load_from_disk(path: impl AsRef<Path>) -> Result<Self> {
        let p = path.as_ref();
        validate_path(p)?;
        if !p.exists() {
            return Err(SunriseError::FileNotFound(p.to_string_lossy().to_string()));
        }
        let content = fs::read_to_string(p)
            .map_err(|e| SunriseError::IoError(e.to_string()))?;
        serde_json::from_str(&content)
            .map_err(|e| SunriseError::InvalidJson(e.to_string()))
    }

    pub fn save_to_disk(&self, path: impl AsRef<Path>) -> Result<()> {
        let p = path.as_ref();
        validate_path(p)?;
        if let Some(parent) = p.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| SunriseError::InvalidJson(e.to_string()))?;
        fs::write(p, json)
            .map_err(|e| SunriseError::IoError(e.to_string()))?;
        Ok(())
    }
}
