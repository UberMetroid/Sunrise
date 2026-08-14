// File: Linux-Server/src/tui/app_state.rs
// Title: Ratatui Installer State Machine with User Choices
// Plain English: Manages component toggles, progress percentage, and Ghost transcript lines.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallPhase {
    SelectOptions,
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
    pub install_server: bool,
    pub install_desktop_shortcut: bool,
    pub selected_option: usize,
    pub should_exit: bool,
}

impl Default for TuiAppState {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiAppState {
    pub fn new() -> Self {
        Self {
            tick: 0,
            progress: 0,
            phase: InstallPhase::SelectOptions,
            logs: vec![GhostLogEntry {
                is_ghost_speech: true,
                title: "Ghost".to_string(),
                detail: "\"Eyes up, Guardian! Select which components you wish to materialize into your sandbox.\"".to_string(),
            }],
            game_path: String::new(),
            package_count: 0,
            install_server: true,
            install_desktop_shortcut: true,
            selected_option: 0,
            should_exit: false,
        }
    }

    pub fn toggle_selected(&mut self) {
        match self.selected_option {
            0 => self.install_server = !self.install_server,
            1 => self.install_desktop_shortcut = !self.install_desktop_shortcut,
            _ => {}
        }
    }

    pub fn next_option(&mut self) {
        self.selected_option = (self.selected_option + 1) % 3;
    }

    pub fn prev_option(&mut self) {
        if self.selected_option == 0 {
            self.selected_option = 2;
        } else {
            self.selected_option -= 1;
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
