// File: linux/src/main.rs
// Title: Sunrise Linux Server & CLI Entrypoint
// Plain English: Command-line interface to start the Sunrise daemon or run integrity diagnostics.

use std::env;
use std::process;

use sunrise_linux::crypto::hash::sha256_hex;
use sunrise_linux::protocol::bap_frame::{BapFrame, BAP_MAGIC};
use sunrise_linux::protocol::opcode::Opcode;
use sunrise_linux::server::tcp_server::SunriseTcpServer;
use sunrise_linux::settings::config::{ServerConfig, SunriseSettings};
use sunrise_linux::state::light_calculator::{calculate_base_light, GearSlots};
use sunrise_linux::SUNRISE_LINUX_VERSION;

fn print_usage(program_name: &str) {
    println!("Sunrise Linux Daemon v{}", SUNRISE_LINUX_VERSION);
    println!("Usage:");
    println!("  {} server [bind_address] [port]   Start the BAP emulation server", program_name);
    println!("  {} test                           Run self-test diagnostics", program_name);
    println!("  {} version                        Print version information", program_name);
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
