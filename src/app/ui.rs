use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app::{App, AppState};

pub fn render_tip(area: Rect, buf: &mut Buffer, app: &App) {
    let style = get_style(app);
    let title = {
        match app.input_queue.as_str() {
            "re" | "ку" => String::from("  Reddit!"),
            "nix" | "тшч" => String::from("  NixOS!"),
            "tr" | "ек" => String::from("󰊿  Translate!"),
            "vk" | "мл" => String::from("  Vk!"),
            "go" | "пщ" => String::from("  Google Ai!"),
            "du" | "вг" => String::from("  Duck Ai!"),
            "2ch" | "2ср" => String::from("󱐋  2ch!"),
            "git" | "пше" => String::from("  GitHub!"),
            "w" | "ц" => String::from("  Whatapp!"),
            "de" | "ву" => String::from("  Deepseek!"),
            "sdo" | "ывщ" => String::from("  Sdo..."),
            _ => match app.app_status {
                super::AppStatus::DuckDuckGo => String::from("󰇥  DuckDuckGo!"),
                super::AppStatus::NixOS => String::from("  NixOS!"),
                super::AppStatus::Translate => String::from("󰊿  Translate!"),
            },
        }
    };

    Paragraph::new(title)
        .alignment(Alignment::Center)
        .style(style)
        .render(area, buf);
}

pub fn render_input(area: Rect, buf: &mut Buffer, app: &App) {
    let style = get_style(app);

    let block = Block::default()
        .borders(Borders::ALL)
        .title_top({
            let output = std::process::Command::new("bash")
                .arg("-c")
                .arg("niri msg -j keyboard-layouts | jq .current_idx")
                .output()
                .expect("error get layout name");

            if output.status.success() {
                let output = String::from_utf8_lossy(&output.stdout).trim().to_string();

                if output == "0" {
                    String::from("English")
                } else {
                    String::from("Russia")
                }
            } else {
                panic!("error get layout name");
            }
        })
        .title_bottom({
            if app.app_state == AppState::Input {
                String::from("input")
            } else {
                String::from("normal")
            }
        })
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    Paragraph::new(String::from("  ") + &app.input_queue.clone())
        .block(block)
        .style(style)
        .render(area, buf);
}

fn get_style(app: &App) -> ratatui::prelude::Style {
    Style {
        fg: {
            if app.app_state != AppState::Normal {
                match app.input_queue.as_str() {
                    "re" | "ку" => Some(Color::LightRed),
                    "nix" | "тшч" => Some(Color::Blue),
                    "tr" | "ек" => Some(Color::White),
                    "vk" | "мл" => Some(Color::LightBlue),
                    "go" | "пщ" => Some(Color::LightBlue),
                    "du" | "вг" => Some(Color::Yellow),
                    "git" | "пше" => Some(Color::LightMagenta),
                    "2ch" | "2ср" => Some(Color::Rgb(254, 145, 18)),
                    "w" | "ц" => Some(Color::LightGreen),
                    "de" | "ву" => Some(Color::LightBlue),
                    "sdo" | "ывщ" => Some(Color::Gray),
                    _ => match app.app_status {
                        super::AppStatus::DuckDuckGo => Some(Color::LightYellow),
                        super::AppStatus::NixOS => Some(Color::Blue),
                        super::AppStatus::Translate => Some(Color::White),
                    },
                }
            } else {
                Some(Color::Gray)
            }
        },
        ..Default::default()
    }
}
