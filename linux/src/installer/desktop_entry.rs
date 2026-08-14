// File: linux/src/installer/desktop_entry.rs
// Title: Desktop Shortcut & systemd Service Generator
// Plain English: Registers desktop entry and user service for the Sunrise BAP daemon.

use std::fs;
use std::path::Path;

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::get_home_dir;

pub struct DesktopIntegration;

impl DesktopIntegration {
    pub fn install_desktop_entry(binary_path: impl AsRef<Path>) -> Result<()> {
        let home = get_home_dir();
        let apps_dir = home.join(".local").join("share").join("applications");
        fs::create_dir_all(&apps_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create applications dir: {}", e)))?;

        let desktop_file = apps_dir.join("sunrise-server.desktop");
        let content = format!(
            "[Desktop Entry]\n\
             Name=Sunrise Emulation Server\n\
             Comment=Local BAP Emulation Server for Project Sunrise\n\
             Exec={} server\n\
             Terminal=true\n\
             Type=Application\n\
             Categories=Game;Development;\n",
            binary_path.as_ref().to_string_lossy()
        );

        fs::write(desktop_file, content)
            .map_err(|e| SunriseError::IoError(format!("Failed to write desktop entry: {}", e)))?;

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

        Ok(())
    }
}
