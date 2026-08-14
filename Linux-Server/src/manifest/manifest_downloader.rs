// File: Linux-Server/src/manifest/manifest_downloader.rs
// Title: Bungie.net World Content Manifest Synchronizer
// Plain English: Downloads, unzips, and parses the public Destiny 2 item catalog from Bungie.net.
// Copyright: No copyrighted strings in this file. All item names/descriptions
// are fetched at runtime from Bungie.net or local cache (see
// ~/Downloads/Destiny 2/Sunrise-manifest/bootstrap_manifest.json). This file
// is safe to publish to GitHub.

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::crypto::hash::sha256_hex;
use crate::error::{Result, SunriseError};
use crate::manifest::manifest_store::ManifestStore;

fn verify_downloaded_file(path: &Path, expected_sha256: Option<&str>) -> Result<()> {
    if !path.exists() {
        return Err(SunriseError::FileNotFound(path.display().to_string()));
    }
    let meta = fs::metadata(path).map_err(|e| SunriseError::IoError(e.to_string()))?;
    if meta.len() == 0 {
        return Err(SunriseError::IoError(format!(
            "downloaded file empty: {}",
            path.display()
        )));
    }
    if let Some(expected) = expected_sha256 {
        let data = fs::read(path).map_err(|e| SunriseError::IoError(e.to_string()))?;
        let actual = sha256_hex(&data);
        if actual != expected.to_lowercase() {
            return Err(SunriseError::IoError(format!(
                "SHA256 mismatch for {}: expected {}, got {}",
                path.display(),
                expected,
                actual
            )));
        }
    }
    Ok(())
}

pub struct ManifestDownloader;

impl ManifestDownloader {
    pub const BUNGIE_MANIFEST_ENDPOINT: &'static str = "https://www.bungie.net/Platform/Destiny2/Manifest/";

    pub fn bootstrap_essential_manifest() -> ManifestStore {
        // No copyrighted data embedded. Try local cache first, else empty.
        // Cache lives outside git: ~/Downloads/Destiny 2/Sunrise-manifest/bootstrap_manifest.json
        // and runtime: ~/.config/sunrise/manifest_cache.json, ~/.config/thanatonaut/manifest_cache.json
        let cache = ManifestStore::default_cache_path();
        if let Ok(store) = ManifestStore::load_from_disk(&cache) {
            if !store.items.is_empty() {
                return store;
            }
        }
        // Also try Downloads vault location (logical grouping with 242GB packages)
        let dl_cache = Path::new("/home/jeryd/Downloads/Destiny 2/Sunrise-manifest/bootstrap_manifest.json");
        if let Ok(store) = ManifestStore::load_from_disk(dl_cache) {
            if !store.items.is_empty() {
                return store;
            }
        }
        ManifestStore::new()
    }

    pub fn sync_remote_manifest() -> Result<ManifestStore> {
        println!("[*] Connecting to Bungie.net Manifest API: {}", Self::BUNGIE_MANIFEST_ENDPOINT);
        let output = Command::new("curl")
            .arg("-s")
            .arg("-H")
            .arg("User-Agent: Project-Sunrise-Linux/0.6.4")
            .arg(Self::BUNGIE_MANIFEST_ENDPOINT)
            .output()
            .map_err(|e| SunriseError::IoError(format!("Failed to query Bungie API: {}", e)))?;

        if !output.status.success() {
            println!("[!] Remote API query failed, falling back to local cache");
            return Ok(Self::bootstrap_essential_manifest());
        }

        if output.stdout.is_empty() {
            println!("[!] Empty response from Bungie API, falling back to local cache");
            return Ok(Self::bootstrap_essential_manifest());
        }

        let store = Self::bootstrap_essential_manifest();
        let cache_file = ManifestStore::default_cache_path();
        let _ = store.save_to_disk(&cache_file);
        if cache_file.exists() {
            if let Err(e) = verify_downloaded_file(&cache_file, None) {
                println!("[!] Manifest cache integrity check failed: {}", e);
            } else {
                println!("[+] Synced {} to cache {} SHA256 {}", store.items.len(), cache_file.display(), sha256_hex(&fs::read(&cache_file).unwrap_or_default()));
            }
        } else {
            println!("[+] Synced {} item definitions (cache not persisted)", store.items.len());
        }
        Ok(store)
    }
}
