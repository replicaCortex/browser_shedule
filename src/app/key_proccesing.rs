const BROWSER: &str = "zen";
const SWAY: &str = "swaymsg";

use super::{App, AppState, AppStatus};

use core::panic;
use std::process::Command;

use ratatui::crossterm::event::{self, KeyEvent, KeyModifiers};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        event::KeyCode::Char(ch) => {
            if app.app_state != AppState::Input {
                match ch {
                    'd' | 'в' => {
                        search(app, "duckduckgo".to_string());
                    }
                    'n' | 'т' => {
                        search(app, "nixos".to_string());
                    }
                    't' | 'е' => {
                        search(app, "translate".to_string());
                    }
                    'i' | 'ш' => app.app_state = AppState::Input,
                    'p' | 'з' => paste(app),

                    'l' | 'д' => move_cursor_right(app),
                    'h' | 'р' => move_cursor_left(app),

                    'I' | 'Ш' => {
                        app.character_index = 0;
                        app.app_state = AppState::Input
                    }
                    'A' | 'Ф' => {
                        app.character_index = app.input_queue.chars().count();
                        app.app_state = AppState::Input
                    }
                    _ => {}
                }
            } else if key.modifiers == KeyModifiers::CONTROL {
                match ch {
                    'l' | 'д' => move_cursor_right(app),
                    'h' | 'р' => move_cursor_left(app),
                    'u' | 'г' => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            app.input_queue = app
                                .input_queue
                                .chars()
                                .skip(app.character_index)
                                .collect::<String>();
                            app.character_index = 0;
                        }
                    }
                    _ => (),
                }
            } else {
                app.input_queue.push(ch);
                move_cursor_right(app)
            }
        }

        event::KeyCode::Esc => {
            if app.app_state == AppState::Input {
                app.app_state = AppState::Normal;
                app.character_index = app.character_index.saturating_sub(1);
            } else {
                app.app_state = AppState::Quit
            }
        }

        event::KeyCode::Backspace => {
            if app.app_state == AppState::Input {
                delete_char(app);
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

fn delete_char(app: &mut App) {
    if app.character_index != 0 {
        let current_index = app.character_index;
        let from_left_to_current_index = current_index - 1;

        let before_char_to_delete = app.input_queue.chars().take(from_left_to_current_index);
        let after_char_to_delete = app.input_queue.chars().skip(current_index);

        app.input_queue = before_char_to_delete.chain(after_char_to_delete).collect();
        move_cursor_left(app);
    }
}

fn move_cursor_left(app: &mut App) {
    let cursor_moved_left = app.character_index.saturating_sub(1);
    app.character_index = clamp_cursor(app, cursor_moved_left);
}

fn move_cursor_right(app: &mut App) {
    let cursor_moved_right = app.character_index.saturating_add(1);
    app.character_index = clamp_cursor(app, cursor_moved_right);
}

fn clamp_cursor(app: &App, new_cursor_pos: usize) -> usize {
    new_cursor_pos.clamp(0, app.input_queue.chars().count())
}

fn duckduckgo(app: &mut App) {
    match app.input_queue.as_str() {
        "go" | "пщ" => {
            queue_commmand("https://aistudio.google.com/prompts/new_chat".to_string());
        }
        "nix" | "тшч" => {
            queue_commmand("https://search.nixos.org/packages?channel=unstable".to_string());
        }
        "tr" | "ек" => {
            queue_commmand("https://translate.google.com/?hl=en".to_string());
        }
        "re" | "ку" => {
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
    let query = "https://search.nixos.org/packages?channel=unstable&from=0&size=50&sort=relevance&type=packages&query=".to_string() + &app.input_queue;

    let command = format!("swaymsg exec 'zen --new-window '{query}''");

    Command::new("bash")
        .arg("-c")
        .arg(command)
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

    // FIXME: don't send text
    let query = format!(
        "https://translate.google.com/?hl=en&sl=en&tl={translate}&text={0}&op=translate",
        app.input_queue
    );

    let command = format!("swaymsg exec 'zen --new-window '{query}''");

    Command::new("bash")
        .arg("-c")
        .arg(command)
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
    let command = format!("swaymsg exec 'zen --new-window '{queue}''");

    Command::new("bash")
        .arg("-c")
        .arg(command)
        .status()
        .expect("panic!");
}

fn search(app: &mut App, search: String) {
    match search.as_str() {
        "duckduckgo" => {
            app.app_status = AppStatus::DuckDuckGo;
            app.app_state = AppState::Input;
            app.character_index = app.input_queue.chars().count();
        }
        "nixos" => {
            app.app_status = AppStatus::NixOS;
            app.app_state = AppState::Input;
            app.character_index = app.input_queue.chars().count();
        }
        "translate" => {
            app.app_status = AppStatus::Translate;
            app.app_state = AppState::Input;
            app.character_index = app.input_queue.chars().count();
        }
        _ => (),
    }
}
