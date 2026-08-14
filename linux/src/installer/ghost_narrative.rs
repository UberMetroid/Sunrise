// File: linux/src/installer/ghost_narrative.rs
// Title: Ghost Companion Narrative & ASCII Art Engine
// Plain English: Renders Destiny 2 storytelling, Ghost dialogues, and ASCII art during installation.

pub const GHOST_ASCII_BANNER: &str = r#"
            /\
           /  \
     /\   / /\ \   /\        PROJECT SUNRISE // VANGUARD PROTOCOL
    /  \ / /  \ \ /  \       ====================================
   <    V | (o) | V   >      "Eyes up, Guardian. We found a signal."
    \  / \ \  / / \  /
     \/   \ \/ /   \/
           \  /
            \/
"#;

pub fn print_banner() {
    println!("\x1b[1;36m{}\x1b[0m", GHOST_ASCII_BANNER);
}

pub fn ghost_speak(dialogue: &str) {
    println!("\x1b[1;33m[Ghost]\x1b[0m \x1b[1;37m\"{}\"\x1b[0m", dialogue);
}

pub fn story_event(action: &str, detail: &str) {
    println!("  \x1b[38;5;51m✦\x1b[0m \x1b[1m{}\x1b[0m: {}", action, detail);
}

pub fn print_prologue() {
    print_banner();
    ghost_speak(
        "Eyes up, Guardian! The Vanguard networks went dark, but I've tracked \
         a stable frequency.",
    );
    ghost_speak(
        "I'm patching our transponder into a local loopback sandbox. Let's \
         get you back into the fight.",
    );
    println!();
}

pub fn print_scan_start() {
    ghost_speak(
        "Scanning the local storage sectors for Destiny 2 package archives \
         and Steam databanks...",
    );
}

pub fn print_game_found(root: &str, pkg_count: usize) {
    story_event(
        "VAULT LOCATED",
        &format!("Found Destiny 2 installation at {}", root),
    );
    if pkg_count > 0 {
        story_event(
            "ARCHIVES VERIFIED",
            &format!("Indexed {} game package entries in the vault", pkg_count),
        );
    }
    ghost_speak(
        "There it is! The game packages are intact. Setting up our secure \
         local coordinates.",
    );
}

pub fn print_backup_made(backup_path: &str) {
    story_event("CORE SAFEGUARD", &format!("Archived original Steam API to {}", backup_path));
    ghost_speak(
        "I safely backed up the original Steam telemetry core. Your original \
         files are safe with me.",
    );
}

pub fn print_epilogue(config_path: &str) {
    println!();
    story_event("CONFIG LOCKED", &format!("Transponder coordinates saved at {}", config_path));
    story_event("BAP LINK", "127.0.0.1:7777 (Ready for Transmat)");
    println!();
    ghost_speak(
        "All systems green, Guardian! The Traveler's light is shining on \
         local loopback.",
    );
    ghost_speak(
        "Start the server with 'sunrise-linux server' whenever you're ready \
         to launch. I'll see you starside!",
    );
    println!();
}
