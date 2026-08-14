// File: linux/src/installer/depot_downloader.rs
// Title: Legacy Destiny 2 Depot Downloader & Modular Vault Provisioner
// Plain English: Downloads core engine binaries (~1.5 GB) and package archives (~75 GB).

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{Result, SunriseError};
use crate::installer::ghost_narrative::*;
use crate::installer::steam_locator::get_home_dir;

pub const DEPOT_DOWNLOADER_URL: &str =
    "https://github.com/SteamRE/DepotDownloader/releases/latest/download/DepotDownloader-linux-x64.zip";

pub const APP_ID: u32 = 1085660;
pub const DEPOT_CORE_ID: u32 = 1085661;
pub const MANIFEST_CORE: &str = "7180122903232116872";
pub const DEPOT_PACKAGES_ID: u32 = 1085662;
pub const MANIFEST_PACKAGES: &str = "2210332166360342287";

pub struct DepotDownloader;

impl DepotDownloader {
    pub fn ensure_binary() -> Result<PathBuf> {
        let home = get_home_dir();
        let bin_dir = home.join(".config").join("sunrise").join("bin");
        fs::create_dir_all(&bin_dir)
            .map_err(|e| SunriseError::IoError(format!("Failed to create bin dir: {}", e)))?;

        let exe_path = bin_dir.join("DepotDownloader");
        if exe_path.exists() {
            return Ok(exe_path);
        }

        let zip_path = bin_dir.join("DepotDownloader.zip");
        animate_spinner("Retrieving DepotDownloader toolkit from GitHub...", 800);

        let curl_status = Command::new("curl")
            .arg("-fsSL")
            .arg(DEPOT_DOWNLOADER_URL)
            .arg("-o")
            .arg(&zip_path)
            .status()
            .map_err(|e| SunriseError::IoError(format!("Failed to download DepotDownloader: {}", e)))?;

        if !curl_status.success() || !zip_path.exists() {
            return Err(SunriseError::IoError(
                "Failed to download DepotDownloader release zip".to_string(),
            ));
        }

        let _ = Command::new("unzip")
            .arg("-o")
            .arg(&zip_path)
            .arg("-d")
            .arg(&bin_dir)
            .status();

        let _ = fs::remove_file(&zip_path);
        if exe_path.exists() {
            let _ = fs::set_permissions(&exe_path, fs::Permissions::from_mode(0o755));
            Ok(exe_path)
        } else {
            Err(SunriseError::FileNotFound(exe_path.display().to_string()))
        }
    }

    pub fn print_manual_instructions(target_dir: &Path) {
        println!("\n\x1b[1;36m[LEGACY DESTINY 2 DEPOT DOWNLOAD COMMANDS]\x1b[0m");
        println!("\x1b[38;5;240m───────────────────────────────────────────────────────────────────────\x1b[0m");
        println!("Target Vault Directory: \x1b[1;32m{}\x1b[0m\n", target_dir.display());
        println!("1. Download Core Game Binaries (Depot {} - ~1.5 GB):", DEPOT_CORE_ID);
        println!(
            "   \x1b[1;33mDepotDownloader -app {} -depot {} -manifest {} -dir \"{}\" -username <STEAM_USER>\x1b[0m\n",
            APP_ID, DEPOT_CORE_ID, MANIFEST_CORE, target_dir.display()
        );
        println!("2. Download Package Assets (Depot {} - ~75 GB):", DEPOT_PACKAGES_ID);
        println!(
            "   \x1b[1;33mDepotDownloader -app {} -depot {} -manifest {} -dir \"{}\" -username <STEAM_USER>\x1b[0m\n",
            APP_ID, DEPOT_PACKAGES_ID, MANIFEST_PACKAGES, target_dir.display()
        );
        println!("\x1b[38;5;240m───────────────────────────────────────────────────────────────────────\x1b[0m");
    }

    pub fn run_interactive_download(target_dir: &Path) -> Result<()> {
        let exe = match Self::ensure_binary() {
            Ok(e) => e,
            Err(_) => {
                Self::print_manual_instructions(target_dir);
                return Ok(());
            }
        };

        println!("\n\x1b[1;36m[LEGACY DESTINY 2 DEPOT DOWNLOADER]\x1b[0m");
        println!("Target Directory: \x1b[1;32m{}\x1b[0m", target_dir.display());
        println!("Choose download scope:");
        println!("  \x1b[1;33m1\x1b[0m. Core Engine Binaries (~1.5 GB - Fast Boot & Server Test)");
        println!("  \x1b[1;33m2\x1b[0m. Full Exploration Vault (~75 GB - Core + All Vaulted Packages)");
        println!("  \x1b[1;33m3\x1b[0m. Package Content Archives Only (~75 GB)\n");

        print!("  \x1b[1;33m[?]\x1b[0m \x1b[1mSelect scope [1/2/3] (default: 1)\x1b[0m: ");
        let _ = std::io::Write::flush(&mut std::io::stdout());

        let mut scope = String::new();
        let _ = std::io::stdin().read_line(&mut scope);
        let scope = scope.trim();

        print!("  \x1b[1;33m[?]\x1b[0m \x1b[1mEnter Steam Username (or press Enter for command help)\x1b[0m: ");
        let _ = std::io::Write::flush(&mut std::io::stdout());

        let mut username = String::new();
        let _ = std::io::stdin().read_line(&mut username);
        let username = username.trim();

        if username.is_empty() {
            Self::print_manual_instructions(target_dir);
            return Ok(());
        }

        match scope {
            "2" => {
                log_info("DEPOT 1/2", "Downloading core game binaries (~1.5 GB)...");
                let _ = Command::new(&exe)
                    .args(["-app", &APP_ID.to_string()])
                    .args(["-depot", &DEPOT_CORE_ID.to_string()])
                    .args(["-manifest", MANIFEST_CORE])
                    .args(["-dir", &target_dir.to_string_lossy()])
                    .args(["-username", username])
                    .status();

                log_info("DEPOT 2/2", "Downloading package content archives (~75 GB)...");
                let _ = Command::new(&exe)
                    .args(["-app", &APP_ID.to_string()])
                    .args(["-depot", &DEPOT_PACKAGES_ID.to_string()])
                    .args(["-manifest", MANIFEST_PACKAGES])
                    .args(["-dir", &target_dir.to_string_lossy()])
                    .args(["-username", username])
                    .status();
            }
            "3" => {
                log_info("DEPOT 2/2", "Downloading package content archives (~75 GB)...");
                let _ = Command::new(&exe)
                    .args(["-app", &APP_ID.to_string()])
                    .args(["-depot", &DEPOT_PACKAGES_ID.to_string()])
                    .args(["-manifest", MANIFEST_PACKAGES])
                    .args(["-dir", &target_dir.to_string_lossy()])
                    .args(["-username", username])
                    .status();
            }
            _ => {
                log_info("DEPOT 1/2", "Downloading core game binaries (~1.5 GB)...");
                let _ = Command::new(&exe)
                    .args(["-app", &APP_ID.to_string()])
                    .args(["-depot", &DEPOT_CORE_ID.to_string()])
                    .args(["-manifest", MANIFEST_CORE])
                    .args(["-dir", &target_dir.to_string_lossy()])
                    .args(["-username", username])
                    .status();
            }
        }

        log_ok("DEPOT VAULT", "Download operations completed");
        Ok(())
    }
}
