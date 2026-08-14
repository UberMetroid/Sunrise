// File: linux/src/installer/desktop_entry.rs
// Title: Application Launcher, Icon, Helper Script & systemd Service Generator
// Plain English: Registers Start Menu entries, systemd daemon service, and auto-start launcher wrapper.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::get_home_dir;

pub const SUNRISE_SVG_ICON: &str = include_str!("../../assets/sunrise-icon.svg");

pub struct DesktopIntegration;

impl DesktopIntegration {
    pub fn install_app_icon() -> Result<()> {
        let home = get_home_dir();
        let icon_dir = home
            .join(".local")
            .join("share")
            .join("icons")
            .join("hicolor")
            .join("scalable")
            .join("apps");

        fs::create_dir_all(&icon_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create icon directory: {}", e)))?;

        let icon_path = icon_dir.join("sunrise.svg");
        fs::write(&icon_path, SUNRISE_SVG_ICON)
            .map_err(|e| SunriseError::IoError(format!("Failed to write app icon: {}", e)))?;

        Ok(())
    }

    pub fn install_game_launcher_wrapper() -> Result<()> {
        let home = get_home_dir();
        let bin_dir = home.join(".local").join("bin");
        fs::create_dir_all(&bin_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create ~/.local/bin: {}", e)))?;

        let wrapper_path = bin_dir.join("sunrise-game");
        let script = r#"#!/usr/bin/env bash
# Project Sunrise Destiny 2 Launcher Wrapper
# Verifies server status before launching Steam

if ! nc -z 127.0.0.1 7777 2>/dev/null && ! ss -tulpn | grep -q ":7777"; then
    echo "[*] Sunrise BAP server is not running. Starting background server..."
    if command -v systemctl &>/dev/null && systemctl --user is-enabled sunrise.service &>/dev/null; then
        systemctl --user start sunrise.service
    else
        nohup "$HOME/.local/bin/sunrise-linux" server >/dev/null 2>&1 &
    fi
    sleep 1
fi

echo "[+] Launching Destiny 2 via Steam (App ID: 1085660)..."
exec steam steam://rungameid/1085660
"#;

        fs::write(&wrapper_path, script)
            .map_err(|e| SunriseError::IoError(format!("Failed to write sunrise-game wrapper: {}", e)))?;
        let _ = fs::set_permissions(&wrapper_path, fs::Permissions::from_mode(0o755));

        Ok(())
    }

    pub fn install_desktop_entry(binary_path: impl AsRef<Path>) -> Result<()> {
        let home = get_home_dir();
        let apps_dir = home.join(".local").join("share").join("applications");

        // Clean up any legacy desktop files if present
        let desktop_dir = home.join("Desktop");
        if desktop_dir.exists() {
            let _ = fs::remove_file(desktop_dir.join("sunrise-server.desktop"));
            let _ = fs::remove_file(desktop_dir.join("destiny2-sunrise.desktop"));
        }

        // Install the vector icon and helper wrapper
        let _ = Self::install_app_icon();
        let _ = Self::install_game_launcher_wrapper();

        fs::create_dir_all(&apps_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create applications dir: {}", e)))?;

        // 1. Sunrise Emulation Server Launcher (Start Menu)
        let server_content = format!(
            "[Desktop Entry]\n\
             Name=Sunrise Emulation Server\n\
             Comment=Local BAP Emulation Server for Project Sunrise\n\
             Exec={} server\n\
             Icon=sunrise\n\
             Terminal=true\n\
             Type=Application\n\
             Categories=Game;Development;\n",
            binary_path.as_ref().to_string_lossy()
        );

        // 2. Destiny 2 Game Launcher (Start Menu)
        let game_content = format!(
            "[Desktop Entry]\n\
             Name=Destiny 2 (Project Sunrise)\n\
             Comment=Launch Destiny 2 via Steam with Project Sunrise Sandbox\n\
             Exec={}/.local/bin/sunrise-game\n\
             Icon=sunrise\n\
             Terminal=false\n\
             Type=Application\n\
             Categories=Game;\n",
            home.display()
        );

        let server_menu = apps_dir.join("sunrise-server.desktop");
        let game_menu = apps_dir.join("destiny2-sunrise.desktop");
        fs::write(&server_menu, &server_content)
            .map_err(|e| SunriseError::IoError(format!("Failed to write server menu entry: {}", e)))?;
        fs::write(&game_menu, &game_content)
            .map_err(|e| SunriseError::IoError(format!("Failed to write game menu entry: {}", e)))?;

        Ok(())
    }

    pub fn install_systemd_service(binary_path: impl AsRef<Path>) -> Result<()> {
        let home = get_home_dir();
        let service_dir = home.join(".config").join("systemd").join("user");
        fs::create_dir_all(&service_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create systemd user dir: {}", e)))?;

        let service_file = service_dir.join("sunrise.service");
        let content = format!(
            "[Unit]\n\
             Description=Project Sunrise Local Emulation Daemon\n\
             After=network.target\n\n\
             [Service]\n\
             Type=simple\n\
             ExecStart={} server\n\
             Restart=on-failure\n\n\
             [Install]\n\
             WantedBy=default.target\n",
            binary_path.as_ref().to_string_lossy()
        );

        fs::write(service_file, content)
            .map_err(|e| SunriseError::IoError(format!("Failed to write systemd service: {}", e)))?;

        // Reload systemd user daemon
        let _ = Command::new("systemctl")
            .arg("--user")
            .arg("daemon-reload")
            .status();

        Ok(())
    }
}
