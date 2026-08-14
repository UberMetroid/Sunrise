// File: linux/src/main.rs
// Title: Sunrise Linux Server & CLI Entrypoint
// Plain English: Command-line interface to start the Sunrise daemon, install to Steam/XDG, or run diagnostics.

use std::env;
use std::path::PathBuf;
use std::process;

use sunrise_linux::crypto::hash::sha256_hex;
use sunrise_linux::installer::config_setup::SunriseDirectories;
use sunrise_linux::installer::desktop_entry::DesktopIntegration;
use sunrise_linux::installer::ghost_narrative::*;
use sunrise_linux::installer::mod_installer::ModInstaller;
use sunrise_linux::installer::steam_locator::search_destiny2_installations;
use sunrise_linux::installer::uninstaller::Uninstaller;
use sunrise_linux::protocol::bap_frame::{BapFrame, BAP_MAGIC};
use sunrise_linux::protocol::opcode::Opcode;
use sunrise_linux::server::tcp_server::SunriseTcpServer;
use sunrise_linux::settings::config::ServerConfig;
use sunrise_linux::state::light_calculator::{calculate_base_light, GearSlots};
use sunrise_linux::state::package_scanner::PackageIndex;
use sunrise_linux::SUNRISE_LINUX_VERSION;

fn print_usage(program_name: &str) {
    print_banner();
    println!("Usage:");
    println!("  {} install                        Detect Steam/Destiny 2 & configure sandbox", program_name);
    println!("  {} index [packages_dir]           Scan & cache Destiny 2 package manifest headers", program_name);
    println!("  {} server [bind_address] [port]   Start the BAP emulation server", program_name);
    println!("  {} test                           Run self-test diagnostics", program_name);
    println!("  {} uninstall                      Restore original game files & remove integration", program_name);
    println!("  {} version                        Print version information", program_name);
}

fn run_install() -> bool {
    print_prologue();
    print_scan_start();
    let installations = search_destiny2_installations();

    if installations.is_empty() {
        print_no_game_found();
        let dirs = SunriseDirectories::default_paths();
        if let Err(e) = dirs.initialize(None) {
            eprintln!("[-] Error initializing config: {}", e);
            return false;
        }
        print_epilogue(&dirs.config_dir.display().to_string());
        return true;
    }

    for inst in &installations {
        print_game_found(&inst.game_root.display().to_string(), 126608);

        // 1. Initialize ~/.config/sunrise
        let dirs = SunriseDirectories::default_paths();
        if let Err(e) = dirs.initialize(Some(&inst.game_root)) {
            eprintln!("[-] Error initializing config directory: {}", e);
            return false;
        }

        // 2. Backup original steam_api64.dll
        if let Ok(backup) = ModInstaller::backup_original_dll(inst) {
            print_backup_made(&backup.display().to_string());
        }

        // 3. Check for compiled Sunrise.dll hook
        let candidate_dlls = [
            PathBuf::from("build-win/Sunrise.dll"),
            PathBuf::from("../build-win/Sunrise.dll"),
            PathBuf::from("Sunrise.dll"),
        ];

        for dll_path in &candidate_dlls {
            if dll_path.exists() {
                if ModInstaller::install_hook_dll(inst, dll_path).is_ok() {
                    story_event("HOOK INSTALLED", &format!("Sunrise.dll -> {}", inst.steam_api_dll.display()));
                    break;
                }
            }
        }
    }

    // Register desktop integration
    if let Ok(current_exe) = env::current_exe() {
        let _ = DesktopIntegration::install_desktop_entry(&current_exe);
        let _ = DesktopIntegration::install_systemd_service(&current_exe);
        story_event("DESKTOP LINK", "Registered ~/.local/share/applications/sunrise-server.desktop");
    }

    let dirs = SunriseDirectories::default_paths();
    print_epilogue(&dirs.config_dir.display().to_string());
    true
}

fn run_uninstall() -> bool {
    let results = Uninstaller::restore_all_game_files();
    for (path, restored) in results {
        if restored {
            story_event("RESTORED", &format!("Original steam_api64.dll in {}", path));
        }
    }
    let _ = Uninstaller::remove_desktop_integration();
    print_uninstall_complete();
    true
}

fn run_index(custom_path: Option<String>) -> bool {
    let packages_path = match custom_path {
        Some(p) => PathBuf::from(p),
        None => {
            let installations = search_destiny2_installations();
            if let Some(first) = installations.first() {
                first.packages_dir.clone()
            } else {
                eprintln!("[-] No Destiny 2 packages directory found to index.");
                return false;
            }
        }
    };

    println!("[*] Scanning package vault: {}", packages_path.display());
    match PackageIndex::scan_directory(&packages_path) {
        Ok(idx) => {
            let dirs = SunriseDirectories::default_paths();
            let cache_file = dirs.cache_dir.join("package_index.json");
            let _ = idx.save_to_cache(&cache_file);
            println!("[+] Indexed {} package files (Total Size: {} bytes)", idx.total_packages, idx.total_bytes);
            println!("[+] Saved package manifest cache to: {}", cache_file.display());
            true
        }
        Err(e) => {
            eprintln!("[-] Indexing error: {}", e);
            false
        }
    }
}

fn run_self_tests() -> bool {
    println!("[*] Running Sunrise Linux Self-Test Diagnostics...");

    // 1. Framing Test
    let payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let frame = BapFrame::new(42, Opcode::Signon, payload.clone());
    let encoded = match frame.to_bytes() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[-] Frame encode failed: {}", e);
            return false;
        }
    };
    assert_eq!(&encoded[..4], &BAP_MAGIC);

    let (decoded, consumed) = match BapFrame::decode(&encoded) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[-] Frame decode failed: {}", e);
            return false;
        }
    };
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.transaction_id, 42);
    assert_eq!(decoded.opcode, Opcode::Signon);
    assert_eq!(decoded.payload, payload);
    println!("  [+] BAP Framing Test: PASSED");

    // 2. Light Calculation Test
    let gear = GearSlots::new(750, 750, 750, 750, 750, 750, 750, 750);
    assert_eq!(calculate_base_light(&gear), 750);
    println!("  [+] Light Calculation Test: PASSED");

    // 3. Cryptographic Hash Test (RFC 6234)
    let digest = sha256_hex(b"sunrise");
    assert_eq!(digest, "e9f2a0186210e30a516d12b001717fc17b1887acad69faf5c2141067f3f6b094");
    println!("  [+] SHA-256 Digest Verification: PASSED");

    println!("[✓] All self-tests passed successfully!");
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).map(|s| s.as_str()).unwrap_or("sunrise-linux");

    if args.len() < 2 {
        print_usage(program);
        process::exit(1);
    }

    match args[1].as_str() {
        "install" => {
            if !run_install() {
                process::exit(1);
            }
        }
        "uninstall" | "restore" => {
            if !run_uninstall() {
                process::exit(1);
            }
        }
        "index" => {
            let custom_path = args.get(2).cloned();
            if !run_index(custom_path) {
                process::exit(1);
            }
        }
        "server" => {
            let bind_addr = args.get(2).cloned().unwrap_or_else(|| "127.0.0.1".to_string());
            let port = args.get(3).and_then(|p| p.parse::<u16>().ok()).unwrap_or(7777);

            let mut config = ServerConfig::default();
            config.bind_address = bind_addr.clone();
            config.port = port;

            println!("Starting Sunrise Linux Server on {}:{}...", bind_addr, port);
            let server = SunriseTcpServer::new(config);
            if let Err(e) = server.run() {
                eprintln!("Server error: {}", e);
                process::exit(1);
            }
        }
        "test" => {
            if !run_self_tests() {
                process::exit(1);
            }
        }
        "version" | "-v" | "--version" => {
            println!("sunrise-linux v{}", SUNRISE_LINUX_VERSION);
        }
        _ => {
            print_usage(program);
            process::exit(1);
        }
    }
}
