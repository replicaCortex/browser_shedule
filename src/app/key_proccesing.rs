const BROWSER: &str = "zen";

use super::{App, AppState, AppStatus};

use core::panic;
use std::process::Command;

use ratatui::crossterm::event::KeyModifiers;
use ratatui::crossterm::event::{self, KeyEvent};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        event::KeyCode::Char('d') => {
            if app.app_state != AppState::Input {
                app.app_status = AppStatus::DuckDuckGo;
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('d');
            }
        }
        event::KeyCode::Char('в') => {
            if app.app_state != AppState::Input {
                app.app_status = AppStatus::DuckDuckGo;
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('в');
            }
        }

        event::KeyCode::Char('n') => {
            if app.app_state != AppState::Input {
                app.app_status = AppStatus::NixOS;
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('n');
            }
        }
        event::KeyCode::Char('т') => {
            if app.app_state != AppState::Input {
                app.app_status = AppStatus::NixOS;
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('т');
            }
        }

        event::KeyCode::Char('t') => {
            if app.app_state != AppState::Input {
                app.app_status = AppStatus::Translate;
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('t');
            }
        }
        event::KeyCode::Char('е') => {
            if app.app_state != AppState::Input {
                app.app_status = AppStatus::Translate;
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('е');
            }
        }

        event::KeyCode::Char('i') => {
            if app.app_state != AppState::Input {
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('i');
            }
        }
        event::KeyCode::Char('ш') => {
            if app.app_state != AppState::Input {
                app.app_state = AppState::Input;
            } else {
                app.input_queue.push('ш');
            }
        }

        event::KeyCode::Char('p') => {
            if app.app_state != AppState::Input {
                paste(app);
            } else {
                app.input_queue.push('р');
            }
        }
        event::KeyCode::Char('з') => {
            if app.app_state != AppState::Input {
                paste(app);
            } else {
                app.input_queue.push('з');
            }
        }
        event::KeyCode::Esc => {
            if app.app_state == AppState::Input {
                app.app_state = AppState::Normal
            } else {
                app.app_state = AppState::Quit
            }
        }

        event::KeyCode::Char('u') if key.modifiers == KeyModifiers::CONTROL => {
            if app.app_state == AppState::Input {
                app.input_queue = "".to_string();
            }
        }
        event::KeyCode::Char('г') if key.modifiers == KeyModifiers::CONTROL => {
            if app.app_state == AppState::Input {
                app.input_queue = "".to_string();
            }
        }

        event::KeyCode::Char(to_insert) => {
            if app.app_state == AppState::Input {
                app.input_queue.push(to_insert);
            }
        }

        event::KeyCode::Enter => match app.app_status {
            AppStatus::DuckDuckGo => duckduckgo(app),
            AppStatus::NixOS => nixos(app),
            AppStatus::Translate => translate(app),
        },

        _ => (),
    }
}

fn duckduckgo(app: &mut App) {
    match app.input_queue.as_str() {
        "go" | "пщ" => {
            queue_commmand("https://aistudio.google.com/prompts/new_chat".to_string());
        }
        "r" | "к" => {
            queue_commmand("https://old.reddit.com/".to_string());
        }
        "de" | "ву" => {
            queue_commmand("https://chat.deepseek.com/".to_string());
        }
        "du" | "вг" => {
            queue_commmand(
                "https://duckduckgo.com/?q=DuckDuckGo+AI+Chat&ia=chat&duckai=1".to_string(),
            );
        }
        "git" | "пше" => {
            queue_commmand("https://github.com/".to_string());
        }
        "2ch" | "2ср" => {
            queue_commmand("https://2ch.su/".to_string());
        }
        "w" | "ц" => {
            queue_commmand("https://web.whatsapp.com/".to_string());
        }
        "sh" | "ыр" => {
            queue_commmand("https://npi-tu.ru/schedule/schedule.html?for=student&faculty=2&year=3&group=%D0%9F%D0%9E%D0%92%D0%B0".to_string());
        }
        "wo" | "цщ" => {
            queue_commmand("https://docs.google.com/document/u/0/".to_string());
        }
        "be" | "иу" => {
            queue_commmand("https://rostov-na-donu.beeline.ru/customers/products/elk/".to_string());
        }
        "sdo" | "ывщ" => {
            queue_commmand("https://sdo.npi-tu.ru/".to_string());
        }
        "vk" | "мл" => {
            queue_commmand("https://vk.com/im".to_string());
        }
        "manga" | "ьфтпф" => {
            queue_commmand("https://mangadex.org/titles/follows".to_string());
        }
        _ => {
            queue_commmand("https://www.duckduckgo.com/search?q=".to_string() + &app.input_queue);
        }
    }

    app.app_state = AppState::Quit;
}

fn nixos(app: &mut App) {
    Command::new(BROWSER)
            .arg("--new-window")
            .arg("https://search.nixos.org/packages?channel=unstable&from=0&size=50&sort=relevance&type=packages&query=".to_string() + &app.input_queue)
            .status()
            .expect("panic!");

    app.app_state = AppState::Quit;
}

fn translate(app: &mut App) {
    let translate = {
        if app.input_queue.chars().any(|c| c.is_ascii_alphabetic()) {
            "ru".to_string()
        } else {
            "en".to_string()
        }
    };

    Command::new(BROWSER)
        .arg("--new-window")
        .arg(
            "https://translate.google.com/?hl=en&sl=en&tl=".to_string()
                + &translate
                + "&text="
                + &app.input_queue
                + "&op=translate",
        )
        .status()
        .expect("panic!");

    app.app_state = AppState::Quit;
}

fn paste(app: &mut App) {
    let output = Command::new("wl-paste").output().expect("wl-paste error");

    if output.status.success() {
        let clipboard_content = String::from_utf8_lossy(&output.stdout);
        app.input_queue = clipboard_content.to_string();
    } else {
        panic!("wl-paste error parse")
    }

    match app.app_status {
        AppStatus::DuckDuckGo => duckduckgo(app),
        AppStatus::NixOS => nixos(app),
        AppStatus::Translate => translate(app),
    }
}

fn queue_commmand(queue: String) {
    Command::new(BROWSER)
        .arg("--new-window")
        .arg(queue)
        .status()
        .expect("panic!");
}
