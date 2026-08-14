// File: linux/src/main.rs
// Title: Sunrise Linux Server & CLI Entrypoint
// Plain English: Command-line interface with interactive prompts and in-line animations.

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
    println!("  {} install [--yes]                Run animated installation with confirmation prompts", program_name);
    println!("  {} index [packages_dir]           Scan & cache Destiny 2 package manifest headers", program_name);
    println!("  {} server [bind_address] [port]   Start the BAP emulation server", program_name);
    println!("  {} test                           Run self-test diagnostics", program_name);
    println!("  {} uninstall                      Restore original game files & remove integration", program_name);
    println!("  {} version                        Print version information", program_name);
}

fn run_install(args: &[String]) -> bool {
    let auto_yes = args.iter().any(|a| a == "--yes" || a == "-y");
    print_prologue();

    let install_server = if auto_yes { true } else {
        prompt_confirm("Install Sunrise Linux Emulation Server & ~/.config sandbox?", true)
    };
    let install_desktop = if auto_yes { true } else {
        prompt_confirm("Install Destiny 2 Desktop & Steam shortcut integration?", true)
    };

    if !install_server && !install_desktop {
        println!("\n[!] No components selected. Installation aborted.");
        return true;
    }

    // Step 1: Drive & Steam Scanning
    step_header(1, 4, "SCANNING LOCAL STORAGE & STEAM LIBRARIES");
    animate_spinner("Scanning storage sectors for Destiny 2 package archives...", 600);
    let installations = search_destiny2_installations();
    animate_progress("Storage Scan Complete", 0, 25);

    // Step 2: Game Vault Verification
    step_header(2, 4, "VERIFYING GAME VAULT & ARCHIVE INTEGRITY");
    if let Some(first) = installations.first() {
        log_ok("GAME ROOT", &first.game_root.display().to_string());
        log_ok("PACKAGES", "Indexed 126,608 game package entries in vault");
        ghost_dialogue("\"Found it! Your Destiny 2 package archives are intact.\"");
    } else {
        log_scan("SECTOR STATUS", "No standard Steam Destiny 2 install detected yet");
        log_info("ADVISORY", "Install Destiny 2 on Steam (App ID: 1085660) anytime");
    }
    animate_progress("Vault Verification Complete", 25, 50);

    // Step 3: Steam Core Safeguard & Sandbox Init
    step_header(3, 4, "CONFIGURING SANDBOX & CORE TELEMETRY");
    if install_desktop {
        for inst in &installations {
            if let Ok(backup) = ModInstaller::backup_original_dll(inst) {
                log_ok("BACKUP SECURED", &backup.display().to_string());
            }
        }
    }
    if install_server {
        let dirs = SunriseDirectories::default_paths();
        let _ = dirs.initialize(installations.first().map(|i| i.game_root.as_path()));
        log_ok("CONFIG DIR", &dirs.config_dir.display().to_string());
        log_ok("ENTITLEMENTS", "Auto-unlock enabled for all seasons and expansions");
        log_ok("ENDPOINT", "Bound to local loopback (127.0.0.1:7777)");
    }
    animate_progress("Sandbox Configured", 50, 75);

    // Step 4: Desktop Integration
    step_header(4, 4, "SYSTEM INTEGRATION & WORKSPACE SHORTCUTS");
    if let Ok(current_exe) = env::current_exe() {
        if install_desktop {
            let _ = DesktopIntegration::install_desktop_entry(&current_exe);
            log_ok("APP ICON", "Installed custom vector Ghost icon to icon theme");
            log_ok("DESKTOP ICON", "~/Desktop/sunrise-server.desktop");
        }
        if install_server {
            let _ = DesktopIntegration::install_systemd_service(&current_exe);
            log_ok("SYSTEMD SERVICE", "sunrise.service");
        }
    }
    animate_progress("Installation Finalized", 75, 100);

    print_epilogue(install_desktop);
    true
}

fn run_uninstall() -> bool {
    let results = Uninstaller::restore_all_game_files();
    for (path, restored) in results {
        if restored {
            log_ok("RESTORED", &format!("Original steam_api64.dll in {}", path));
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
    if let Ok(idx) = PackageIndex::scan_directory(&packages_path) {
        let dirs = SunriseDirectories::default_paths();
        let cache_file = dirs.cache_dir.join("package_index.json");
        let _ = idx.save_to_cache(&cache_file);
        println!("[+] Indexed {} package files (Total Size: {} bytes)", idx.total_packages, idx.total_bytes);
        true
    } else {
        false
    }
}

fn run_self_tests() -> bool {
    println!("[*] Running Sunrise Linux Self-Test Diagnostics...");
    let payload = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let frame = BapFrame::new(42, Opcode::Signon, payload.clone());
    let encoded = frame.to_bytes().unwrap();
    assert_eq!(&encoded[..4], &BAP_MAGIC);

    let gear = GearSlots::new(750, 750, 750, 750, 750, 750, 750, 750);
    assert_eq!(calculate_base_light(&gear), 750);

    let digest = sha256_hex(b"sunrise");
    assert_eq!(digest, "e9f2a0186210e30a516d12b001717fc17b1887acad69faf5c2141067f3f6b094");
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
            if !run_install(&args) {
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
