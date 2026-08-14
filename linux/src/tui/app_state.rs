// File: linux/src/tui/app_state.rs
// Title: Ratatui Installer State Machine
// Plain English: Manages progress percentage, Ghost transcript lines, and animation ticks.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallPhase {
    InitialScan,
    FoundVault,
    SecuringCore,
    WritingConfig,
    DesktopSetup,
    Finished,
}

#[derive(Debug, Clone)]
pub struct GhostLogEntry {
    pub is_ghost_speech: bool,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct TuiAppState {
    pub tick: usize,
    pub progress: u16,
    pub phase: InstallPhase,
    pub logs: Vec<GhostLogEntry>,
    pub game_path: String,
    pub package_count: usize,
    pub should_exit: bool,
}

impl TuiAppState {
    pub fn new() -> Self {
        Self {
            tick: 0,
            progress: 0,
            phase: InstallPhase::InitialScan,
            logs: vec![GhostLogEntry {
                is_ghost_speech: true,
                title: "Ghost".to_string(),
                detail: "\"Eyes up, Guardian. Connecting to your local frequency...\"".to_string(),
            }],
            game_path: String::new(),
            package_count: 0,
            should_exit: false,
        }
    }

    pub fn add_speech(&mut self, text: &str) {
        self.logs.push(GhostLogEntry {
            is_ghost_speech: true,
            title: "Ghost".to_string(),
            detail: format!("\"{}\"", text),
        });
    }

    pub fn add_event(&mut self, title: &str, detail: &str) {
        self.logs.push(GhostLogEntry {
            is_ghost_speech: false,
            title: title.to_string(),
            detail: detail.to_string(),
        });
    }

    pub fn advance_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }
}
