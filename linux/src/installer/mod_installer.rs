// File: linux/src/installer/mod_installer.rs
// Title: Client DLL Backup & Hook Installer
// Plain English: Backs up original steam_api64.dll and places Sunrise.dll in the game directory.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::Destiny2Paths;

pub struct ModInstaller;

impl ModInstaller {
    pub fn backup_original_dll(paths: &Destiny2Paths) -> Result<PathBuf> {
        let original_backup = paths.bin_x64_dir.join("steam_api64_original.dll");
        let active_dll = &paths.steam_api_dll;

        if !active_dll.exists() {
            return Err(SunriseError::FileNotFound(
                active_dll.to_string_lossy().to_string(),
            ));
        }

        // Only backup if the backup file does not already exist
        if !original_backup.exists() {
            fs::copy(active_dll, &original_backup).map_err(|e| {
                SunriseError::IoError(format!("Failed to backup steam_api64.dll: {}", e))
            })?;
        }

        Ok(original_backup)
    }

    pub fn install_hook_dll(paths: &Destiny2Paths, sunrise_dll_path: impl AsRef<Path>) -> Result<()> {
        let src_dll = sunrise_dll_path.as_ref();
        if !src_dll.exists() {
            return Err(SunriseError::FileNotFound(
                src_dll.to_string_lossy().to_string(),
            ));
        }

        // Ensure original is backed up first
        Self::backup_original_dll(paths)?;

        // Copy Sunrise.dll as steam_api64.dll
        fs::copy(src_dll, &paths.steam_api_dll).map_err(|e| {
            SunriseError::IoError(format!("Failed to install Sunrise.dll: {}", e))
        })?;

        Ok(())
    }

    pub fn restore_original_dll(paths: &Destiny2Paths) -> Result<()> {
        let original_backup = paths.bin_x64_dir.join("steam_api64_original.dll");
        if !original_backup.exists() {
            return Err(SunriseError::FileNotFound(
                original_backup.to_string_lossy().to_string(),
            ));
        }

        fs::copy(&original_backup, &paths.steam_api_dll).map_err(|e| {
            SunriseError::IoError(format!("Failed to restore original steam_api64.dll: {}", e))
        })?;

        Ok(())
    }
}
