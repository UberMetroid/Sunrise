// File: Linux-Server/src/installer/steam_locator.rs
// Title: Steam & Destiny 2 Directory Locator
// Plain English: Locates Steam installation directories and Destiny 2 game paths on Linux.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub const DESTINY_2_APP_ID: u32 = 1085660;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destiny2Paths {
    pub game_root: PathBuf,
    pub packages_dir: PathBuf,
    pub bin_x64_dir: PathBuf,
    pub steam_api_dll: PathBuf,
}

impl Destiny2Paths {
    pub fn from_root(root: impl AsRef<Path>) -> Option<Self> {
        let game_root = root.as_ref().to_path_buf();
        let packages_dir = game_root.join("packages");
        let bin_x64_dir = game_root.join("bin").join("x64");
        let steam_api_dll = bin_x64_dir.join("steam_api64.dll");

        if game_root.join("destiny2.exe").exists() {
            Some(Self {
                game_root,
                packages_dir,
                bin_x64_dir,
                steam_api_dll,
            })
        } else {
            None
        }
    }
}

pub fn get_home_dir() -> PathBuf {
    match env::var("HOME") {
        Ok(h) if !h.is_empty() && Path::new(&h).is_absolute() => PathBuf::from(h),
        _ => {
            eprintln!("[!] HOME not set or not absolute, refusing to probe '.'");
            PathBuf::from("/tmp")
        }
    }
}

pub fn search_destiny2_installations() -> Vec<Destiny2Paths> {
    let mut found = Vec::new();
    let home = get_home_dir();

    let candidate_steam_roots = [
        home.join(".local/share/Steam"),
        home.join(".steam/steam"),
        home.join(".steam/root"),
        home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
        home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
        home.join("snap/steam/common/.steam"),
        home.join(".steam/debian-installation"),
    ];

    for steam_root in &candidate_steam_roots {
        if !steam_root.exists() {
            continue;
        }

        // Direct common path
        let direct_destiny_path = steam_root
            .join("steamapps")
            .join("common")
            .join("Destiny 2");

        if let Some(paths) = Destiny2Paths::from_root(&direct_destiny_path) {
            if !found.contains(&paths) {
                found.push(paths);
            }
        }

        // Check libraryfolders.vdf for additional library paths
        let vdf_path = steam_root.join("steamapps").join("libraryfolders.vdf");
        if vdf_path.exists() {
            for custom_lib in parse_libraryfolders_vdf(&vdf_path) {
                let custom_destiny = custom_lib
                    .join("steamapps")
                    .join("common")
                    .join("Destiny 2");
                if let Some(paths) = Destiny2Paths::from_root(&custom_destiny) {
                    if !found.contains(&paths) {
                        found.push(paths);
                    }
                }
            }
        }
    }

    found
}

pub fn parse_libraryfolders_vdf(vdf_path: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(content) = fs::read_to_string(vdf_path) {
        for line in content.lines() {
            let trimmed = line.trim();
            // Match "path" " /some/dir " in both old and new VDF formats
            if trimmed.contains("\"path\"") {
                if let Some(start) = trimmed.find("\"path\"") {
                    let rest = &trimmed[start + 6..];
                    // Find first quoted string after "path"
                    let mut in_quote = false;
                    let mut current = String::new();
                    let mut found: Option<String> = None;
                    for ch in rest.chars() {
                        if ch == '"' {
                            if in_quote {
                                if !current.trim().is_empty() {
                                    found = Some(current.clone());
                                }
                                break;
                            }
                            in_quote = true;
                            current.clear();
                        } else if in_quote {
                            current.push(ch);
                        }
                    }
                    if let Some(path_str) = found {
                        let pb = PathBuf::from(path_str.trim());
                        if pb.exists() && !paths.contains(&pb) {
                            paths.push(pb);
                        }
                    }
                }
            }
        }
    }
    paths
}
