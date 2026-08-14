// File: linux/src/installer/ghost_narrative.rs
// Title: Ghost Companion Narrative & In-Line Terminal Animation Engine
// Plain English: Renders Ghost ASCII art, clean border dialogue, and in-line animations.

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

pub const GHOST_ASCII_BANNER: &str = r#"
                 /\
                /  \
           /\  / /\ \  /\           PROJECT SUNRISE // LINUX FOUNDRY
          /  \/ /  \ \/  \          ================================
         / /\  / /\ \  /\ \         "Eyes up, Guardian. We found a signal."
        / /  \/ /  \ \/  \ \
       < <    | ( O ) |   > >       Offline BAP Emulation & Sandbox
        \ \  /\ \  / /\  / /
         \ \/  \ \/ /  \/ /
          \  /\ \  / /\  /
           \/  \ \/ /  \/
                \  /
                 \/
"#;

pub fn print_banner() {
    println!("\x1b[1;36m{}\x1b[0m", GHOST_ASCII_BANNER);
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for raw_line in text.lines() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            lines.push(String::new());
            continue;
        }

        let words: Vec<&str> = trimmed.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
    lines
}

pub fn ghost_dialogue(speech: &str) {
    println!("\x1b[1;33m╭─ Ghost ─────────────────────────────────────────────────────────────╮\x1b[0m");
    let wrapped = wrap_text(speech, 66);
    for line in wrapped {
        println!("  \x1b[1;37m{}\x1b[0m", line);
    }
    println!("\x1b[1;33m╰─────────────────────────────────────────────────────────────────────╯\x1b[0m");
}

pub fn step_header(step_idx: usize, total: usize, title: &str) {
    println!(
        "\n\x1b[1;36m[STEP {}/{}]\x1b[0m \x1b[1m{}\x1b[0m",
        step_idx, total, title
    );
    println!("\x1b[38;5;240m───────────────────────────────────────────────────────────────────────\x1b[0m");
}

pub fn log_ok(item: &str, detail: &str) {
    println!("  \x1b[1;32m[  OK  ]\x1b[0m \x1b[1m{:<20}\x1b[0m {}", item, detail);
}

pub fn log_scan(item: &str, detail: &str) {
    println!("  \x1b[1;33m[ SCAN ]\x1b[0m \x1b[1m{:<20}\x1b[0m {}", item, detail);
}

pub fn log_info(item: &str, detail: &str) {
    println!("  \x1b[1;36m[ INFO ]\x1b[0m \x1b[1m{:<20}\x1b[0m {}", item, detail);
}

pub fn animate_spinner(label: &str, duration_ms: u64) {
    let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠇", "⠏"];
    let start = Instant::now();
    let mut idx = 0;
    while start.elapsed().as_millis() < duration_ms as u128 {
        print!("\r  \x1b[1;33m[{}]\x1b[0m \x1b[1m{}\x1b[0m", spinner[idx % spinner.len()], label);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(60));
        idx += 1;
    }
    print!("\r\x1b[K");
    let _ = io::stdout().flush();
}

pub fn animate_progress(step_name: &str, from_pct: usize, to_pct: usize) {
    for pct in from_pct..=to_pct {
        let filled = pct / 5;
        let empty = 20 - filled;
        let bar: String = "█".repeat(filled) + &"░".repeat(empty);
        print!(
            "\r  \x1b[1;36m[PROGRESS]\x1b[0m \x1b[1;32m[{}]\x1b[0m {:>3}% - {}",
            bar, pct, step_name
        );
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(12));
    }
    println!();
}

pub fn prompt_confirm(question: &str, default_yes: bool) -> bool {
    let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("  \x1b[1;33m[?]\x1b[0m \x1b[1m{} \x1b[1;36m{}\x1b[0m: ", question, suffix);
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim().to_lowercase();
        if trimmed.is_empty() {
            default_yes
        } else {
            trimmed == "y" || trimmed == "yes"
        }
    } else {
        default_yes
    }
}

pub fn print_prologue() {
    print_banner();
    ghost_dialogue(
        "\"The Red War took the main Bungie relay offline, but I've isolated a stable local frequency. I will guide you through setting up our offline BAP sandbox step-by-step. Stand by.\"",
    );
}

pub fn print_proton_box() {
    println!("\n\x1b[1;33m╔═════════════════════════════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[1;36m║  ✦ VANGUARD STEAM LAUNCH PARAMETER (REQUIRED FOR PROTON / WINE) ✦   ║\x1b[0m");
    println!("\x1b[1;33m╠═════════════════════════════════════════════════════════════════════╣\x1b[0m");
    println!("\x1b[1;37m║  In Steam, right-click Destiny 2 -> Properties -> Launch Options:   ║\x1b[0m");
    println!("║                                                                     ║");
    println!("║    \x1b[1;32mWINEDLLOVERRIDES=\"steam_api64=n,b\" %command%\x1b[0m                     ║");
    println!("║                                                                     ║");
    println!("\x1b[1;33m╚═════════════════════════════════════════════════════════════════════╝\x1b[0m");
}

pub fn print_epilogue(has_desktop: bool) {
    if has_desktop {
        print_proton_box();
    }
    println!();
    ghost_dialogue(
        "\"All selected systems are green, Guardian! The Traveler's light is shining on local loopback. Launch 'thanatonaut server' whenever you're ready to transmat into the sandbox. I'll see you starside!\"",
    );
    println!();
}

pub fn print_uninstall_complete() {
    print_banner();
    ghost_dialogue(
        "\"Reverting local loopback modifications and restoring original Vanguard telemetry... Your game installation is back in pristine state.\"",
    );
    log_ok("RESTORATION", "Original steam_api64.dll restored in game folders");
    log_ok("CLEANUP", "Removed start menu entries, vector icon, and service");
    println!();
}
