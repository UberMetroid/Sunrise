// File: linux/src/tui/views.rs
// Title: Ratatui Interface Views & Animated HUD Layout
// Plain English: Renders animated Ghost radar, storyline transcript, and progress gauge.

use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app_state::{InstallPhase, TuiAppState};
use crate::tui::ghost_art::{get_animated_ghost_frame, get_radar_sweep};

pub fn render_ui(frame: &mut Frame, state: &TuiAppState) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header Title
            Constraint::Min(12),   // Main Split (Ghost Shell + Transcript)
            Constraint::Length(4), // Progress Gauge
            Constraint::Length(3), // Controls / Footer
        ])
        .split(size);

    render_header(frame, chunks[0]);
    render_main_split(frame, chunks[1], state);
    render_gauge(frame, chunks[2], state);
    render_footer(frame, chunks[3], state);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let title_line = Line::from(vec![
        Span::styled("PROJECT SUNRISE ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("// VANGUARD LINUX SANDBOX FOUNDRY", Style::default().fg(Color::Yellow)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(title_line)
        .alignment(Alignment::Center)
        .block(block);

    frame.render_widget(paragraph, area);
}

fn render_main_split(frame: &mut Frame, area: Rect, state: &TuiAppState) {
    let sub_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(40)])
        .split(area);

    render_ghost_hud(frame, sub_chunks[0], state);
    render_transcript(frame, sub_chunks[1], state);
}

fn render_ghost_hud(frame: &mut Frame, area: Rect, state: &TuiAppState) {
    let ghost_lines = get_animated_ghost_frame(state.tick);
    let sweep = get_radar_sweep(state.tick);

    let mut lines = Vec::new();
    lines.push(Line::from(""));
    for gl in ghost_lines {
        lines.push(Line::from(Span::styled(
            *gl,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        sweep,
        Style::default().fg(Color::Yellow),
    )));

    let block = Block::default()
        .title(" Ghost Radar ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(block);

    frame.render_widget(paragraph, area);
}

fn render_transcript(frame: &mut Frame, area: Rect, state: &TuiAppState) {
    let mut text_lines = Vec::new();

    // Show last 10 log entries
    let start_idx = state.logs.len().saturating_sub(10);
    for entry in &state.logs[start_idx..] {
        if entry.is_ghost_speech {
            text_lines.push(Line::from(vec![
                Span::styled(" [Ghost] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(&entry.detail, Style::default().fg(Color::White)),
            ]));
        } else {
            text_lines.push(Line::from(vec![
                Span::styled("  ✦ ", Style::default().fg(Color::Cyan)),
                Span::styled(format!("{}: ", entry.title), Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
                Span::styled(&entry.detail, Style::default().fg(Color::Gray)),
            ]));
        }
        text_lines.push(Line::from(""));
    }

    let block = Block::default()
        .title(" Companion Storyline & Telemetry ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(text_lines)
        .wrap(Wrap { trim: true })
        .block(block);

    frame.render_widget(paragraph, area);
}

fn render_gauge(frame: &mut Frame, area: Rect, state: &TuiAppState) {
    let label = format!("{}% - {}", state.progress, match state.phase {
        InstallPhase::InitialScan => "Scanning Drive Sectors",
        InstallPhase::FoundVault => "Verifying Destiny 2 Game Vault",
        InstallPhase::SecuringCore => "Safeguarding Steam API Core",
        InstallPhase::WritingConfig => "Initializing ~/.config/sunrise Sandbox",
        InstallPhase::DesktopSetup => "Registering Desktop Icon & Service",
        InstallPhase::Finished => "Installation Complete - Ready for Transmat",
    });

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(" Foundry Installation Progress ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green)),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .percent(state.progress)
        .label(label);

    frame.render_widget(gauge, area);
}

fn render_footer(frame: &mut Frame, area: Rect, state: &TuiAppState) {
    let footer_text = if state.phase == InstallPhase::Finished {
        vec![
            Span::styled(" WINEDLLOVERRIDES=\"steam_api64=n,b\" %command% ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" | Press [Enter] to Launch Server | [Esc/q] to Exit", Style::default().fg(Color::White)),
        ]
    } else {
        vec![
            Span::styled(" [Installing...] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("Ghost is reconstructing your local loopback transponder. Press [Esc] to cancel.", Style::default().fg(Color::DarkGray)),
        ]
    };

    let paragraph = Paragraph::new(Line::from(footer_text))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    frame.render_widget(paragraph, area);
}
