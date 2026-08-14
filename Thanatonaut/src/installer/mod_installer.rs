// File: linux/src/installer/mod_installer.rs
// Title: Client DLL & Anti-Cheat Launcher Bypass Manager
// Plain English: Backs up original binaries, installs proxy hook, and bypasses BattlEye launcher.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::{get_home_dir, Destiny2Paths};

pub const DEFAULT_HOOK_URL: &str =
    "https://github.com/stanuwu/Sunrise/releases/latest/download/steam_api64.dll";

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

    pub fn resolve_hook_dll() -> Result<PathBuf> {
        let home = get_home_dir();
        let cache_dll = home.join(".config").join("thanatonaut").join("steam_api64.dll");

        let candidates = [
            PathBuf::from("build-win/Sunrise.dll"),
            PathBuf::from("../build-win/Sunrise.dll"),
            PathBuf::from("Sunrise.dll"),
            cache_dll.clone(),
        ];

        for path in &candidates {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // If not found locally, download official release artifact
        let download_url = env::var("SUNRISE_HOOK_URL")
            .unwrap_or_else(|_| DEFAULT_HOOK_URL.to_string());

        if let Some(parent) = cache_dll.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let status = Command::new("curl")
            .arg("-fsSL")
            .arg(&download_url)
            .arg("-o")
            .arg(&cache_dll)
            .status()
            .map_err(|e| SunriseError::IoError(format!("Failed to spawn curl: {}", e)))?;

        if !status.success() || !cache_dll.exists() {
            return Err(SunriseError::IoError(format!(
                "Failed to download Sunrise proxy DLL from {}",
                download_url
            )));
        }

        Ok(cache_dll)
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

        // Copy Sunrise proxy DLL as steam_api64.dll
        fs::copy(src_dll, &paths.steam_api_dll).map_err(|e| {
            SunriseError::IoError(format!("Failed to install proxy steam_api64.dll: {}", e))
        })?;

        Ok(())
    }

    pub fn ensure_proxy_hook(paths: &Destiny2Paths) -> Result<PathBuf> {
        Self::backup_original_dll(paths)?;
        let hook_src = Self::resolve_hook_dll()?;
        Self::install_hook_dll(paths, &hook_src)?;
        Ok(paths.steam_api_dll.clone())
    }

    pub fn backup_and_bypass_launcher(paths: &Destiny2Paths) -> Result<()> {
        let launcher_exe = paths.game_root.join("destiny2launcher.exe");
        let launcher_backup = paths.game_root.join("destiny2launcher_original.exe");
        let game_exe = paths.game_root.join("destiny2.exe");

        if !game_exe.exists() {
            return Err(SunriseError::FileNotFound(
                game_exe.to_string_lossy().to_string(),
            ));
        }

        if launcher_exe.exists() && !launcher_backup.exists() {
            fs::copy(&launcher_exe, &launcher_backup).map_err(|e| {
                SunriseError::IoError(format!("Failed to backup destiny2launcher.exe: {}", e))
            })?;
        }

        // Direct destiny2launcher.exe to execute destiny2.exe directly
        fs::copy(&game_exe, &launcher_exe).map_err(|e| {
            SunriseError::IoError(format!("Failed to bypass BattlEye launcher: {}", e))
        })?;

        Ok(())
    }

    pub fn restore_launcher(paths: &Destiny2Paths) -> Result<()> {
        let launcher_exe = paths.game_root.join("destiny2launcher.exe");
        let launcher_backup = paths.game_root.join("destiny2launcher_original.exe");

        if launcher_backup.exists() {
            fs::copy(&launcher_backup, &launcher_exe).map_err(|e| {
                SunriseError::IoError(format!("Failed to restore destiny2launcher.exe: {}", e))
            })?;
        }

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
