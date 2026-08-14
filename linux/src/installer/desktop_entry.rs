// File: linux/src/installer/desktop_entry.rs
// Title: Desktop Shortcut & systemd Service Generator
// Plain English: Registers desktop entry on ~/Desktop and user applications menu.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::error::{Result, SunriseError};
use crate::installer::steam_locator::get_home_dir;

pub struct DesktopIntegration;

impl DesktopIntegration {
    pub fn install_desktop_entry(binary_path: impl AsRef<Path>) -> Result<()> {
        let home = get_home_dir();
        let apps_dir = home.join(".local").join("share").join("applications");
        let desktop_dir = home.join("Desktop");

        fs::create_dir_all(&apps_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create applications dir: {}", e)))?;

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

        // 1. Install to system applications menu
        let menu_file = apps_dir.join("sunrise-server.desktop");
        fs::write(&menu_file, &content)
            .map_err(|e| SunriseError::IoError(format!("Failed to write menu entry: {}", e)))?;

        // 2. Install directly to ~/Desktop if available
        if desktop_dir.exists() {
            let direct_desktop_file = desktop_dir.join("sunrise-server.desktop");
            fs::write(&direct_desktop_file, &content)
                .map_err(|e| SunriseError::IoError(format!("Failed to write desktop icon: {}", e)))?;
            // Mark executable so Linux desktop environments can launch it directly
            let _ = fs::set_permissions(&direct_desktop_file, fs::Permissions::from_mode(0o755));
        }

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
