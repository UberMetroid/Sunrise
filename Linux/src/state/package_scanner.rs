// File: linux/src/state/package_scanner.rs
// Title: Game Package Scanner & Manifest Indexer
// RFC Reference: RFC 8259 (JSON Data Interchange Format)
// Plain English: Scans Destiny 2 package archives and caches package metadata.

use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use crate::error::{Result, SunriseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageMetadata {
    pub package_id: u16,
    pub filename: String,
    pub file_size: u64,
    pub entry_count: u32,
}

#[derive(Debug, Clone, Default)]
pub struct PackageIndex {
    pub total_packages: usize,
    pub total_bytes: u64,
    pub packages: Vec<PackageMetadata>,
}

impl PackageIndex {
    pub fn scan_directory(packages_dir: impl AsRef<Path>) -> Result<Self> {
        let dir = packages_dir.as_ref();
        if !dir.exists() || !dir.is_dir() {
            return Err(SunriseError::FileNotFound(
                dir.to_string_lossy().to_string(),
            ));
        }

        let mut index = PackageIndex::default();
        let read_dir = fs::read_dir(dir).map_err(|e| {
            SunriseError::IoError(format!("Failed to read packages directory: {}", e))
        })?;

        for entry_result in read_dir {
            let entry = match entry_result {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "pkg" {
                    if let Ok(meta) = Self::inspect_package(&path) {
                        index.total_bytes += meta.file_size;
                        index.total_packages += 1;
                        index.packages.push(meta);
                    }
                }
            }
        }

        Ok(index)
    }

    pub fn inspect_package(pkg_path: &Path) -> Result<PackageMetadata> {
        let mut file = File::open(pkg_path).map_err(|e| {
            SunriseError::IoError(format!("Failed to open package {}: {}", pkg_path.display(), e))
        })?;

        let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);
        let filename = pkg_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        // Extract package ID from filename (e.g. w64_bootflow_01a3_0.pkg -> 0x01a3)
        let package_id = Self::parse_package_id_from_name(&filename);

        // Read entry count from header if available (at least 16 bytes)
        let mut header = [0u8; 16];
        let mut entry_count = 0u32;
        if file.read_exact(&mut header).is_ok() {
            // First 4 bytes or offset 4 typically store entry count in Tiger pkg
            entry_count = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        }

        Ok(PackageMetadata {
            package_id,
            filename,
            file_size,
            entry_count,
        })
    }

    pub fn parse_package_id_from_name(name: &str) -> u16 {
        // Look for 4-digit hex substring in filename
        let parts: Vec<&str> = name.split('_').collect();
        for part in parts {
            if part.len() == 4 {
                if let Ok(val) = u16::from_str_radix(part, 16) {
                    return val;
                }
            }
        }
        0
    }

    pub fn save_to_cache(&self, cache_file: impl AsRef<Path>) -> Result<()> {
        let json_content = format!(
            "{{\n  \"total_packages\": {},\n  \"total_bytes\": {},\n  \"indexed_at\": \"offline\"\n}}",
            self.total_packages, self.total_bytes
        );
        fs::write(cache_file.as_ref(), json_content)
            .map_err(|e| SunriseError::IoError(format!("Failed to write cache: {}", e)))?;
        Ok(())
    }
}
