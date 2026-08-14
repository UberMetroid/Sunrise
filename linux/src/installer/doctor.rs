// File: linux/src/installer/doctor.rs
// Title: Sunrise Linux Doctor & Health Diagnostics
// Plain English: Inspects system health, game archives, proxy hook status, and port availability.

use std::net::TcpStream;

use crate::installer::config_setup::SunriseDirectories;
use crate::installer::ghost_narrative::{ghost_dialogue, log_info, log_ok, log_scan, print_banner};
use crate::installer::steam_locator::{get_home_dir, search_destiny2_installations};

pub struct SunriseDoctor;

impl SunriseDoctor {
    pub fn check_status() {
        print_banner();
        println!("\x1b[1;36m[DAEMON STATUS CHECK]\x1b[0m");
        println!("\x1b[38;5;240m───────────────────────────────────────────────────────────────────────\x1b[0m");

        match TcpStream::connect("127.0.0.1:7777") {
            Ok(_) => {
                log_ok("PORT 7777", "Listening (127.0.0.1:7777 - ONLINE)");
                log_ok("DAEMON STATE", "Sunrise BAP Emulation Server is ACTIVE");
                ghost_dialogue("\"The local BAP beacon is active and accepting connections, Guardian.\"");
            }
            Err(_) => {
                log_scan("PORT 7777", "Not responding (OFFLINE)");
                log_info("DAEMON STATE", "Server is not currently running");
                ghost_dialogue("\"The server is offline. Start it with 'sunrise-linux server' whenever you're ready.\"");
            }
        }
        println!();
    }

    pub fn run_diagnostics() -> bool {
        print_banner();
        println!("\x1b[1;36m[PROJECT SUNRISE // VANGUARD DOCTOR DIAGNOSTICS]\x1b[0m");
        println!("\x1b[38;5;240m───────────────────────────────────────────────────────────────────────\x1b[0m");

        let mut all_healthy = true;

        // 1. Check Steam & Game Installation
        let installations = search_destiny2_installations();
        if let Some(first) = installations.first() {
            log_ok("DESTINY 2 VAULT", &first.game_root.display().to_string());
            if first.packages_dir.exists() {
                log_ok("PACKAGES DIR", &first.packages_dir.display().to_string());
            } else {
                log_scan("PACKAGES DIR", "Missing packages directory");
                all_healthy = false;
            }

            // 2. Check Backup DLL & Proxy Hook
            let backup_path = first.bin_x64_dir.join("steam_api64_original.dll");
            if backup_path.exists() {
                log_ok("CORE SAFEGUARD", "Original steam_api64_original.dll intact");
            } else {
                log_scan("CORE SAFEGUARD", "No backup DLL found (run 'sunrise-linux install')");
            }

            if first.steam_api_dll.exists() {
                let orig_len = backup_path.metadata().map(|m| m.len()).unwrap_or(0);
                let cur_len = first.steam_api_dll.metadata().map(|m| m.len()).unwrap_or(0);
                if orig_len > 0 && cur_len != orig_len {
                    log_ok("PROXY HOOK", "Project Sunrise steam_api64.dll is ACTIVE");
                } else {
                    log_scan("PROXY HOOK", "Vanilla DLL active (run 'sunrise-linux install')");
                }
            }
        } else {
            log_scan("DESTINY 2 VAULT", "No standard installation detected in Steam paths");
            all_healthy = false;
        }

        // 3. Check ~/.config/sunrise Sandbox
        let dirs = SunriseDirectories::default_paths();
        if dirs.config_dir.exists() && dirs.config_file.exists() {
            log_ok("SANDBOX CONFIG", &dirs.config_file.display().to_string());
        } else {
            log_scan("SANDBOX CONFIG", "Missing ~/.config/sunrise/config.json");
            all_healthy = false;
        }

        // 4. Check Desktop Shortcuts & Icons
        let home = get_home_dir();
        let icon_file = home.join(".local/share/icons/hicolor/scalable/apps/sunrise.svg");
        let desktop_file = home.join("Desktop/destiny2-sunrise.desktop");
        let menu_file = home.join(".local/share/applications/destiny2-sunrise.desktop");

        if icon_file.exists() {
            log_ok("VECTOR ICON", &icon_file.display().to_string());
        } else {
            log_scan("VECTOR ICON", "Missing icon file");
        }

        if desktop_file.exists() || menu_file.exists() {
            log_ok("DESKTOP LAUNCHER", "Registered in desktop environment");
        } else {
            log_scan("DESKTOP LAUNCHER", "Not installed (run 'sunrise-linux install')");
        }

        // 5. Check Port Availability
        match TcpStream::connect("127.0.0.1:7777") {
            Ok(_) => {
                log_ok("PORT 7777", "Active / Listening");
            }
            Err(_) => {
                log_info("PORT 7777", "Available for binding");
            }
        }

        println!("\x1b[38;5;240m───────────────────────────────────────────────────────────────────────\x1b[0m");
        if all_healthy {
            println!("\x1b[1;32m[✓] VANGUARD SYSTEM DIAGNOSTICS: ALL SYSTEMS OPERATIONAL\x1b[0m\n");
        } else {
            println!("\x1b[1;33m[!] VANGUARD SYSTEM DIAGNOSTICS: ADVISORIES DETECTED\x1b[0m\n");
        }

        all_healthy
    }
}
