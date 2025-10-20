use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app::{App, AppState};

pub fn render_tip(area: Rect, buf: &mut Buffer, app: &App) {
    let style = get_style(app);
    let title = {
        match app.app_status {
            super::AppStatus::DuckDuckGo => String::from("󰇥  DuckDuckGo!"),
            super::AppStatus::NixOS => String::from("  NixOS!"),
            super::AppStatus::Translate => String::from("󰊿  Translate!"),
        }
    };

    Paragraph::new(title)
        .alignment(Alignment::Center)
        .style(style)
        .render(area, buf);
}

pub fn render_input(area: Rect, buf: &mut Buffer, app: &App) {
    let horizontal_chunks = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(2),
        Constraint::Fill(1),
    ])
    .split(area);

    let style = get_style(app);

    let block = Block::default()
        .borders(Borders::ALL)
        .title_top({
            if app.app_state == AppState::Input {
                String::from("input")
            } else {
                String::from("normal")
            }
        })
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    Paragraph::new(app.input_queue.clone())
        .alignment(Alignment::Center)
        .block(block)
        .style(style)
        .render(horizontal_chunks[1], buf);
}

fn get_style(app: &App) -> ratatui::prelude::Style {
    Style {
        fg: {
            if app.app_state != AppState::Normal {
                match app.app_status {
                    super::AppStatus::DuckDuckGo => Some(Color::LightYellow),
                    super::AppStatus::NixOS => Some(Color::LightBlue),
                    super::AppStatus::Translate => Some(Color::LightRed),
                }
            } else {
                Some(Color::Gray)
            }
        },
        ..Default::default()
    }
}
