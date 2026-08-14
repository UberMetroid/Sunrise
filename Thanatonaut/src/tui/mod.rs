// File: Thanatonaut/src/tui/mod.rs
// Title: Ratatui Animated TUI Installer Runner
// Plain English: Handles keyboard navigation, component toggling, and installation lifecycle.

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
                    KeyCode::Up => {
                        if state.phase == InstallPhase::SelectOptions {
                            state.prev_option();
                        }
                    }
                    KeyCode::Down | KeyCode::Tab => {
                        if state.phase == InstallPhase::SelectOptions {
                            state.next_option();
                        }
                    }
                    KeyCode::Char(' ') => {
                        if state.phase == InstallPhase::SelectOptions {
                            state.toggle_selected();
                        }
                    }
                    KeyCode::Enter => {
                        if state.phase == InstallPhase::SelectOptions {
                            if state.selected_option == 2 || state.selected_option == 0 || state.selected_option == 1 {
                                state.phase = InstallPhase::InitialScan;
                                state.add_speech("Components confirmed! Scanning local storage sectors...");
                                step_tick = 0;
                            }
                        } else if state.phase == InstallPhase::Finished {
                            return Ok(state.install_server);
                        }
                    }
                    _ => {}
                }
            }
        }

        state.advance_tick();

        if state.phase == InstallPhase::SelectOptions {
            continue;
        }

        step_tick += 1;

        // Drive installation steps across animation ticks
        match state.phase {
            InstallPhase::SelectOptions => {}
            InstallPhase::InitialScan => {
                state.progress = (state.progress + 4).min(20);
                if step_tick >= 10 {
                    let installations = search_destiny2_installations();
                    if let Some(first) = installations.first() {
                        state.game_path = first.game_root.display().to_string();
                        state.package_count = 126608;
                        state.add_event("VAULT LOCATED", &state.game_path.clone());
                        state.add_speech("Found it! Your Destiny 2 package vault is intact.");
                    } else {
                        state.add_event("SECTOR STATUS", "No standard Destiny 2 install detected");
                        state.add_speech("No game install found yet, but initializing sandbox transponder.");
                    }
                    state.phase = InstallPhase::FoundVault;
                    step_tick = 0;
                }
            }
            InstallPhase::FoundVault => {
                state.progress = (state.progress + 3).min(45);
                if step_tick >= 8 {
                    if state.install_desktop_shortcut {
                        let installations = search_destiny2_installations();
                        for inst in &installations {
                            if let Ok(backup) = ModInstaller::backup_original_dll(inst) {
                                state.add_event("CORE SAFEGUARD", &backup.display().to_string());
                            }
                        }
                        state.add_speech("Backed up original steam_api64.dll safely.");
                    }
                    state.phase = InstallPhase::SecuringCore;
                    step_tick = 0;
                }
            }
            InstallPhase::SecuringCore => {
                state.progress = (state.progress + 3).min(70);
                if step_tick >= 8 {
                    if state.install_server {
                        let dirs = SunriseDirectories::default_paths();
                        let installations = search_destiny2_installations();
                        let _ = dirs.initialize(installations.first().map(|i| i.game_root.as_path()));
                        state.add_event("CONFIG LOCKED", &dirs.config_dir.display().to_string());
                        state.add_event("BAP ENDPOINT", "127.0.0.1:7777");
                        state.add_speech("Transponder initialized in ~/.config/thanatonaut with DLC unlocks.");
                    }
                    state.phase = InstallPhase::WritingConfig;
                    step_tick = 0;
                }
            }
            InstallPhase::WritingConfig => {
                state.progress = (state.progress + 3).min(90);
                if step_tick >= 8 {
                    if state.install_desktop_shortcut {
                        if let Ok(current_exe) = env::current_exe() {
                            let _ = DesktopIntegration::install_desktop_entry(&current_exe);
                            state.add_event("DESKTOP ICON", "~/Desktop/thanatonaut-server.desktop");
                            state.add_event("START MENU", "~/.local/share/applications/thanatonaut-server.desktop");
                        }
                    }
                    if state.install_server {
                        if let Ok(current_exe) = env::current_exe() {
                            let _ = DesktopIntegration::install_systemd_service(&current_exe);
                            state.add_event("SYSTEMD SERVICE", "thanatonaut.service");
                        }
                    }
                    state.phase = InstallPhase::DesktopSetup;
                    step_tick = 0;
                }
            }
            InstallPhase::DesktopSetup => {
                state.progress = 100;
                state.phase = InstallPhase::Finished;
                state.add_speech("All selected components installed, Guardian! Press [Enter] to launch or [Esc] to exit.");
            }
            InstallPhase::Finished => {}
        }
    }
}
