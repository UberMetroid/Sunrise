// File: linux/tests/test_installer.rs
// Title: Installer, Uninstaller & Scanner E-M Verification Proofs
// Plain English: Tests Steam directory scanning, library VDF parsing, and package header inspection.

use std::fs;
use sunrise_linux::installer::config_setup::SunriseDirectories;
use sunrise_linux::installer::steam_locator::{parse_libraryfolders_vdf, Destiny2Paths};
use sunrise_linux::installer::uninstaller::Uninstaller;
use sunrise_linux::state::package_scanner::PackageIndex;

#[test]
fn test_config_directory_initialization() {
    let temp_dir = std::env::temp_dir().join("sunrise_test_config");
    let dirs = SunriseDirectories {
        config_dir: temp_dir.clone(),
        config_file: temp_dir.join("config.json"),
        cache_dir: temp_dir.join("cache"),
        profiles_dir: temp_dir.join("profiles"),
    };

    dirs.initialize(None).expect("Config initialization should succeed");

    assert!(dirs.config_dir.exists());
    assert!(dirs.config_file.exists());
    assert!(dirs.cache_dir.exists());
    assert!(dirs.profiles_dir.exists());

    // Cleanup
    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_parse_mock_libraryfolders_vdf() {
    let temp_vdf = std::env::temp_dir().join("mock_libraryfolders.vdf");
    let content = r#"
"libraryfolders"
{
    "0"
    {
        "path"    "/tmp"
        "label"   ""
    }
}
"#;
    fs::write(&temp_vdf, content).unwrap();

    let paths = parse_libraryfolders_vdf(&temp_vdf);
    assert!(!paths.is_empty());
    assert_eq!(paths[0], std::path::PathBuf::from("/tmp"));

    let _ = fs::remove_file(temp_vdf);
}

#[test]
fn test_destiny2_paths_rejection_on_missing_binary() {
    let temp_empty = std::env::temp_dir().join("empty_destiny_dir");
    let _ = fs::create_dir_all(&temp_empty);

    assert!(Destiny2Paths::from_root(&temp_empty).is_none());

    let _ = fs::remove_dir_all(temp_empty);
}

#[test]
fn test_package_scanner_mock_package() {
    let temp_pkg_dir = std::env::temp_dir().join("mock_packages");
    let _ = fs::create_dir_all(&temp_pkg_dir);

    let pkg_file = temp_pkg_dir.join("w64_test_01a3_0.pkg");
    let mock_header = [0u8; 32];
    fs::write(&pkg_file, &mock_header).unwrap();

    let index = PackageIndex::scan_directory(&temp_pkg_dir).unwrap();
    assert_eq!(index.total_packages, 1);
    assert_eq!(index.packages[0].package_id, 0x01a3);

    let _ = fs::remove_dir_all(temp_pkg_dir);
}

#[test]
fn test_uninstaller_remove_integration() {
    let result = Uninstaller::remove_desktop_integration();
    assert!(result.is_ok());
}
