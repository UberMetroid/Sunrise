// File: linux/src/installer/ghost_narrative.rs
// Title: Ghost Companion Narrative & Terminal Interface Engine
// Plain English: Guides the Guardian step-by-step with Ghost dialogues, progress badges, and UI boxes.

pub const GHOST_ASCII_BANNER: &str = r#"
            /\
           /  \
     /\   / /\ \   /\        PROJECT SUNRISE // LINUX FOUNDRY
    /  \ / /  \ \ /  \       ================================
   <    V | (o) | V   >      "Eyes up, Guardian. We found a signal."
    \  / \ \  / / \  /
     \/   \ \/ /   \/
           \  /
            \/
"#;

pub fn print_banner() {
    println!("\x1b[1;36m{}\x1b[0m", GHOST_ASCII_BANNER);
}

pub fn ghost_dialogue(speech: &str) {
    println!("\x1b[1;33m╭─ Ghost ─────────────────────────────────────────────────────────────╮\x1b[0m");
    for line in speech.lines() {
        println!("\x1b[1;33m│\x1b[0m  \x1b[1;37m{}\x1b[0m", line);
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

pub fn print_prologue() {
    print_banner();
    ghost_dialogue(
        "\"The Red War took the main Bungie relay offline, but I've isolated\n \
         a stable local frequency. I will guide you through setting up our\n \
         offline BAP sandbox step-by-step. Stand by.\"",
    );
}

pub fn print_step1_scan() {
    step_header(1, 5, "SCANNING LOCAL STORAGE & STEAM LIBRARIES");
    ghost_dialogue(
        "\"Searching your Linux drive sectors for Steam installations and\n \
         parsing libraryfolders.vdf for Destiny 2 package vaults...\"",
    );
}

pub fn print_step2_game_found(root: &str, pkg_count: usize) {
    step_header(2, 5, "VERIFYING GAME VAULT & ARCHIVE INTEGRITY");
    log_ok("GAME ROOT", root);
    log_ok("PACKAGES", &format!("Indexed {} game package entries in vault", pkg_count));
    ghost_dialogue(
        "\"Found it! Your Destiny 2 package archives are intact. We can build\n \
         the emulation sandbox directly on top of these local assets.\"",
    );
}

pub fn print_step2_no_game() {
    step_header(2, 5, "VERIFYING GAME VAULT & ARCHIVE INTEGRITY");
    log_scan("SECTOR STATUS", "No standard Steam Destiny 2 install detected yet");
    log_info("ADVISORY", "Install Destiny 2 on Steam (App ID: 1085660) anytime");
    ghost_dialogue(
        "\"I didn't find Destiny 2 installed yet, Guardian, but don't worry.\n \
         I'm initializing our standalone transponder anyway. Whenever you\n \
         install the game, simply re-run this script to link your files.\"",
    );
}

pub fn print_step3_backup(backup_path: &str) {
    step_header(3, 5, "SAFEGUARDING STEAM TELEMETRY CORE");
    log_ok("BACKUP SECURED", backup_path);
    ghost_dialogue(
        "\"I've created an immutable backup of your original steam_api64.dll.\n \
         Your original game files are 100% safe and can be restored anytime.\"",
    );
}

pub fn print_step4_config(config_path: &str) {
    step_header(4, 5, "INITIALIZING XDG TRANSPONDER & SANDBOX");
    log_ok("CONFIG DIR", config_path);
    log_ok("ENTITLEMENTS", "Auto-unlock enabled for all seasons and expansions");
    log_ok("ENDPOINT", "Bound to local loopback (127.0.0.1:7777)");
    ghost_dialogue(
        "\"Writing your offline profile, sandbox settings, and package cache\n \
         into ~/.config/sunrise. All Vanguard DLC keys are unlocked.\"",
    );
}

pub fn print_step5_desktop(desktop_icon: &str, service_name: &str) {
    step_header(5, 5, "SYSTEM INTEGRATION & WORKSPACE SHORTCUTS");
    log_ok("APP ICON", "Installed custom vector Ghost icon to icon theme");
    log_ok("DESKTOP ICON", desktop_icon);
    log_ok("SYSTEMD SERVICE", service_name);
    ghost_dialogue(
        "\"I've placed a launcher shortcut directly on your Desktop and\n \
         registered a background systemd service for hands-free launch.\"",
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

pub fn print_epilogue() {
    print_proton_box();
    println!();
    ghost_dialogue(
        "\"All systems are green, Guardian! The Traveler's light is shining on\n \
         local loopback. Launch 'sunrise-linux server' whenever you're ready\n \
         to transmat into the sandbox. I'll see you starside!\"",
    );
    println!();
}

pub fn print_uninstall_complete() {
    print_banner();
    ghost_dialogue(
        "\"Reverting local loopback modifications and restoring original\n \
         Vanguard telemetry... Your game installation is back in pristine state.\"",
    );
    log_ok("RESTORATION", "Original steam_api64.dll restored in game folders");
    log_ok("CLEANUP", "Removed desktop icon, start menu entry, and service");
    println!();
}
