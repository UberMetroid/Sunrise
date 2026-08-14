// File: linux/src/installer/uninstaller.rs
// Title: Sunrise Linux Uninstaller & Restore Manager
// Plain English: Restores original Steam API binaries and cleans up desktop/systemd entries.

use std::fs;

use crate::error::{Result, SunriseError};
use crate::installer::mod_installer::ModInstaller;
use crate::installer::steam_locator::{get_home_dir, search_destiny2_installations};

pub struct Uninstaller;

impl Uninstaller {
    pub fn restore_all_game_files() -> Vec<(String, bool)> {
        let mut results = Vec::new();
        let installations = search_destiny2_installations();

        for inst in &installations {
            let path_str = inst.game_root.display().to_string();
            let dll_restored = ModInstaller::restore_original_dll(inst).is_ok();
            let _ = ModInstaller::restore_launcher(inst);
            results.push((path_str, dll_restored));
        }

        results
    }

    pub fn remove_desktop_integration() -> Result<()> {
        let home = get_home_dir();

        // 1. Remove Desktop icons
        let desktop_file = home.join("Desktop").join("sunrise-server.desktop");
        let game_desktop = home.join("Desktop").join("destiny2-sunrise.desktop");
        if desktop_file.exists() {
            let _ = fs::remove_file(desktop_file);
        }
        if game_desktop.exists() {
            let _ = fs::remove_file(game_desktop);
        }

        // 2. Remove application menu entries
        let menu_file = home
            .join(".local")
            .join("share")
            .join("applications")
            .join("sunrise-server.desktop");
        let game_menu = home
            .join(".local")
            .join("share")
            .join("applications")
            .join("destiny2-sunrise.desktop");
        if menu_file.exists() {
            let _ = fs::remove_file(menu_file);
        }
        if game_menu.exists() {
            let _ = fs::remove_file(game_menu);
        }

        // 3. Remove systemd user service
        let service_file = home
            .join(".config")
            .join("systemd")
            .join("user")
            .join("sunrise.service");
        if service_file.exists() {
            let _ = fs::remove_file(service_file);
        }

        // 4. Remove local binary link
        let bin_link = home.join(".local").join("bin").join("sunrise-linux");
        if bin_link.exists() {
            let _ = fs::remove_file(bin_link);
        }

        // 5. Remove application icon
        let icon_file = home
            .join(".local")
            .join("share")
            .join("icons")
            .join("hicolor")
            .join("scalable")
            .join("apps")
            .join("sunrise.svg");
        if icon_file.exists() {
            let _ = fs::remove_file(icon_file);
        }

        Ok(())
    }

    pub fn purge_config_directory(purge_data: bool) -> Result<()> {
        if purge_data {
            let home = get_home_dir();
            let config_dir = home.join(".config").join("sunrise");
            if config_dir.exists() {
                fs::remove_dir_all(config_dir).map_err(|e| {
                    SunriseError::IoError(format!("Failed to purge ~/.config/sunrise: {}", e))
                })?;
            }
        }
        Ok(())
    }
}
