// File: Linux-Server/src/tui/ghost_art.rs
// Title: Animated Ghost ASCII Shell Frames
// Plain English: Generates rotating radar sweeps and pulsing Ghost shell frames.

pub fn get_animated_ghost_frame(tick: usize) -> &'static [&'static str] {
    match tick % 4 {
        0 => &[
            "          /\\          ",
            "         /  \\         ",
            "   /\\   / /\\ \\   /\\   ",
            "  /  \\ / /  \\ \\ /  \\  ",
            " <    V | (o) | V   > ",
            "  \\  / \\ \\  / / \\  /  ",
            "   \\/   \\ \\/ /   \\/   ",
            "         \\  /         ",
            "          \\/          ",
        ],
        1 => &[
            "          /\\          ",
            "         /  \\         ",
            "   /\\   / /\\ \\   /\\   ",
            "  /  \\ / /  \\ \\ /  \\  ",
            " <    V | (•) | V   > ",
            "  \\  / \\ \\  / / \\  /  ",
            "   \\/   \\ \\/ /   \\/   ",
            "         \\  /         ",
            "          \\/          ",
        ],
        2 => &[
            "          /\\          ",
            "         /  \\         ",
            "   /\\   / /\\ \\   /\\   ",
            "  /  \\ / /  \\ \\ /  \\  ",
            " <    V | (O) | V   > ",
            "  \\  / \\ \\  / / \\  /  ",
            "   \\/   \\ \\/ /   \\/   ",
            "         \\  /         ",
            "          \\/          ",
        ],
        _ => &[
            "          /\\          ",
            "         /  \\         ",
            "   /\\   / /\\ \\   /\\   ",
            "  /  \\ / /  \\ \\ /  \\  ",
            " <    V | (-) | V   > ",
            "  \\  / \\ \\  / / \\  /  ",
            "   \\/   \\ \\/ /   \\/   ",
            "         \\  /         ",
            "          \\/          ",
        ],
    }
}

pub fn get_radar_sweep(tick: usize) -> &'static str {
    let sweeps = [
        "⠋ SCANNING SECTOR ALPHA...",
        "⠙ VERIFYING STEAM VAULT...",
        "⠹ DECRYPTING PKG MANIFEST...",
        "⠸ LOCKING LOCAL TRANSPONDER...",
        "⠼ SAFEGUARDING TELEMETRY...",
        "⠴ SYNCHRONIZING BAP CHANNEL...",
        "⠦ FINALIZING FOUNDRY BUILD...",
        "⠇ READY FOR TRANSMAT...",
    ];
    sweeps[tick % sweeps.len()]
}
