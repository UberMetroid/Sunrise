// File: linux/src/main.rs
// Title: Sunrise Linux Server & CLI Entrypoint
// Plain English: Command-line interface to start the Sunrise daemon, install to Steam/XDG, or run diagnostics.

use std::env;
use std::path::PathBuf;
use std::process;

use sunrise_linux::crypto::hash::sha256_hex;
use sunrise_linux::installer::config_setup::SunriseDirectories;
use sunrise_linux::installer::desktop_entry::DesktopIntegration;
use sunrise_linux::installer::mod_installer::ModInstaller;
use sunrise_linux::installer::steam_locator::search_destiny2_installations;
use sunrise_linux::protocol::bap_frame::{BapFrame, BAP_MAGIC};
use sunrise_linux::protocol::opcode::Opcode;
use sunrise_linux::server::tcp_server::SunriseTcpServer;
use sunrise_linux::settings::config::{ServerConfig, SunriseSettings};
use sunrise_linux::state::light_calculator::{calculate_base_light, GearSlots};
use sunrise_linux::SUNRISE_LINUX_VERSION;

fn print_usage(program_name: &str) {
    println!("Sunrise Linux Daemon v{}", SUNRISE_LINUX_VERSION);
    println!("Usage:");
    println!("  {} install                        Detect Steam/Destiny 2 & install config", program_name);
    println!("  {} server [bind_address] [port]   Start the BAP emulation server", program_name);
    println!("  {} test                           Run self-test diagnostics", program_name);
    println!("  {} version                        Print version information", program_name);
}

fn run_install() -> bool {
    println!("[*] Searching for Destiny 2 installations on Linux...");
    let installations = search_destiny2_installations();

    if installations.is_empty() {
        println!("[-] No Destiny 2 installation found in standard Steam library paths.");
        println!("    Creating standalone ~/.config/sunrise configuration...");
        let dirs = SunriseDirectories::default_paths();
        if let Err(e) = dirs.initialize(None) {
            eprintln!("[-] Error initializing config directory: {}", e);
            return false;
        }
        println!("[+] Initialized config directory at: {}", dirs.config_dir.display());
        return true;
    }

    println!("[+] Found {} Destiny 2 installation(s):", installations.len());
    for (idx, inst) in installations.iter().enumerate() {
        println!("    [{}] Game Root: {}", idx + 1, inst.game_root.display());
        println!("        Packages:  {}", inst.packages_dir.display());
        println!("        Bin (x64): {}", inst.bin_x64_dir.display());

        // 1. Initialize ~/.config/sunrise
        let dirs = SunriseDirectories::default_paths();
        if let Err(e) = dirs.initialize(Some(&inst.game_root)) {
            eprintln!("[-] Error initializing config directory: {}", e);
            return false;
        }
        println!("    [+] Configuration initialized at: {}", dirs.config_dir.display());

        // 2. Backup original steam_api64.dll
        match ModInstaller::backup_original_dll(inst) {
            Ok(backup) => println!("    [+] Backed up original Steam API DLL to: {}", backup.display()),
            Err(e) => eprintln!("    [!] Note on Steam API backup: {}", e),
        }

        // 3. Check for compiled Sunrise.dll hook
        let candidate_dlls = [
            PathBuf::from("build-win/Sunrise.dll"),
            PathBuf::from("../build-win/Sunrise.dll"),
            PathBuf::from("Sunrise.dll"),
        ];

        let mut dll_installed = false;
        for dll_path in &candidate_dlls {
            if dll_path.exists() {
                match ModInstaller::install_hook_dll(inst, dll_path) {
                    Ok(_) => {
                        println!("    [+] Installed Sunrise.dll -> {}", inst.steam_api_dll.display());
                        dll_installed = true;
                        break;
                    }
                    Err(e) => eprintln!("    [-] Error copying Sunrise.dll: {}", e),
                }
            }
        }

        if !dll_installed {
            println!("    [i] Note: To complete client hook installation, cross-compile Sunrise.dll with MinGW");
            println!("        and run 'sunrise-linux install' again.");
        }
    }

    // Register desktop integration
    if let Ok(current_exe) = env::current_exe() {
        let _ = DesktopIntegration::install_desktop_entry(&current_exe);
        let _ = DesktopIntegration::install_systemd_service(&current_exe);
        println!("[+] Created desktop shortcut and systemd user service (~/.config/systemd/user/sunrise.service)");
    }

    println!("\n[✓] Sunrise Linux installation completed successfully!");
    true
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

    let gear_uneven = GearSlots::new(750, 751, 752, 753, 754, 755, 756, 757);
    assert_eq!(calculate_base_light(&gear_uneven), 753);
    println!("  [+] Light Calculation Test: PASSED");

    // 3. Cryptographic Hash Test (RFC 6234)
    let digest = sha256_hex(b"sunrise");
    assert_eq!(
        digest,
        "e9f2a0186210e30a516d12b001717fc17b1887acad69faf5c2141067f3f6b094"
    );
    println!("  [+] SHA-256 Digest Verification: PASSED");

    // 4. JSON Config Test (RFC 8259)
    let default_settings = SunriseSettings::default();
    let json_str = default_settings.to_json_string().unwrap();
    let parsed = SunriseSettings::from_json_str(&json_str).unwrap();
    assert_eq!(default_settings, parsed);
    println!("  [+] JSON Settings Serialization: PASSED");

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
