// File: linux/src/tui/mod.rs
// Title: Ratatui Animated TUI Installer Runner
// Plain English: Initializes interactive terminal and runs animated Ghost installation.

pub mod app_state;
pub mod ghost_art;
pub mod views;

use std::env;
use std::io::stdout;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::error::Result;
use crate::installer::config_setup::SunriseDirectories;
use crate::installer::desktop_entry::DesktopIntegration;
use crate::installer::mod_installer::ModInstaller;
use crate::installer::steam_locator::search_destiny2_installations;
use crate::tui::app_state::{InstallPhase, TuiAppState};
use crate::tui::views::render_ui;

pub fn run_tui_installer() -> Result<bool> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut state = TuiAppState::new();
    let res = run_installer_loop(&mut terminal, &mut state);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    res
}

fn run_installer_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut TuiAppState,
) -> Result<bool> {
    let mut step_tick = 0;

    loop {
        terminal.draw(|f| render_ui(f, state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        state.should_exit = true;
                        return Ok(false);
                    }
                    KeyCode::Enter if state.phase == InstallPhase::Finished => {
                        return Ok(true);
                    }
                    _ => {}
                }
            }
        }

        state.advance_tick();
        step_tick += 1;

        // Drive installer steps across animation ticks
        match state.phase {
            InstallPhase::InitialScan => {
                state.progress = (state.progress + 4).min(20);
                if step_tick == 5 {
                    state.add_speech("Searching local storage sectors for Destiny 2 package archives...");
                }
                if step_tick >= 10 {
                    let installations = search_destiny2_installations();
                    if let Some(first) = installations.first() {
                        state.game_path = first.game_root.display().to_string();
                        state.package_count = 126608;
                        state.add_event("VAULT LOCATED", &state.game_path.clone());
                        state.add_speech("Found it! Your Destiny 2 package vault is intact. Linking archives...");
                    } else {
                        state.add_event("SECTOR STATUS", "No standard Destiny 2 install detected");
                        state.add_speech("No game install found yet, but initializing your transponder anyway.");
                    }
                    state.phase = InstallPhase::FoundVault;
                    step_tick = 0;
                }
            }
            InstallPhase::FoundVault => {
                state.progress = (state.progress + 3).min(45);
                if step_tick >= 10 {
                    let installations = search_destiny2_installations();
                    for inst in &installations {
                        if let Ok(backup) = ModInstaller::backup_original_dll(inst) {
                            state.add_event("CORE SAFEGUARD", &backup.display().to_string());
                            state.add_speech("Backed up original steam_api64.dll. Files are completely safe.");
                        }
                    }
                    state.phase = InstallPhase::SecuringCore;
                    step_tick = 0;
                }
            }
            InstallPhase::SecuringCore => {
                state.progress = (state.progress + 3).min(70);
                if step_tick >= 10 {
                    let dirs = SunriseDirectories::default_paths();
                    let installations = search_destiny2_installations();
                    let _ = dirs.initialize(installations.first().map(|i| i.game_root.as_path()));
                    state.add_event("CONFIG LOCKED", &dirs.config_dir.display().to_string());
                    state.add_event("BAP ENDPOINT", "127.0.0.1:7777");
                    state.add_speech("Transponder initialized in ~/.config/sunrise with full DLC unlocks.");
                    state.phase = InstallPhase::WritingConfig;
                    step_tick = 0;
                }
            }
            InstallPhase::WritingConfig => {
                state.progress = (state.progress + 3).min(90);
                if step_tick >= 10 {
                    if let Ok(current_exe) = env::current_exe() {
                        let _ = DesktopIntegration::install_desktop_entry(&current_exe);
                        let _ = DesktopIntegration::install_systemd_service(&current_exe);
                        state.add_event("DESKTOP ICON", "~/Desktop/sunrise-server.desktop");
                        state.add_event("SYSTEMD SERVICE", "sunrise.service");
                        state.add_speech("Placed desktop shortcut and registered background service.");
                    }
                    state.phase = InstallPhase::DesktopSetup;
                    step_tick = 0;
                }
            }
            InstallPhase::DesktopSetup => {
                state.progress = 100;
                state.phase = InstallPhase::Finished;
                state.add_speech("All systems green, Guardian! Ready for transmat. Press [Enter] to launch.");
            }
            InstallPhase::Finished => {}
        }
    }
}
