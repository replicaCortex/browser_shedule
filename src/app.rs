mod key_proccesing;

mod ui;

use std::io::stdout;
use std::sync::mpsc;
use std::thread;

use color_eyre::eyre::Result;
use crossterm::execute;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::{self, KeyEvent};
use ratatui::prelude::*;

use crate::app::key_proccesing::paste;

pub enum AppEvent {
    Key(KeyEvent),
    Paste(),
}

#[derive(Default, PartialEq)]
enum AppState {
    Normal,
    #[default]
    Input,
    Quit,
}

#[derive(Default)]
enum AppStatus {
    #[default]
    DuckDuckGo,
    NixOS,
    Translate,
}

#[derive(Default)]
pub struct App {
    app_state: AppState,
    app_status: AppStatus,

    input_queue: String,
    character_index: usize,
}

impl App {
    pub fn run(
        mut self,
        mut terminal: DefaultTerminal,
        rc: mpsc::Receiver<AppEvent>,
    ) -> Result<()> {
        while self.is_runing() {
            terminal.draw(|frame| Self::render_ui(&self, frame))?;

            match rc.try_recv() {
                Ok(AppEvent::Key(key_event)) => {
                    key_proccesing::handle_key_event(&mut self, key_event)
                }

                Ok(AppEvent::Paste()) => paste(&mut self),
                Err(_) => (),
            }

            check_search_engine(&mut self);
        }

        Ok(())
    }

    fn render_ui(&self, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();

        let vertical_chunks = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

        let horizontal_chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
        .split(vertical_chunks[3]);

        ui::render_tip(vertical_chunks[1], buf, self);
        ui::render_input(horizontal_chunks[1], buf, self);

        match self.app_state {
            AppState::Input => execute!(stdout(), crossterm::cursor::SetCursorStyle::SteadyBar)
                .expect("error set cursor style (steadebar)"),
            AppState::Normal => execute!(stdout(), crossterm::cursor::SetCursorStyle::SteadyBlock)
                .expect("error set cursor style (steadyblock)"),
            _ => (),
        };

        frame.set_cursor_position(Position {
            x: horizontal_chunks[1].x + self.character_index as u16 + 4,
            y: horizontal_chunks[1].y + 1,
        });
    }

    fn is_runing(&self) -> bool {
        self.app_state == AppState::Normal || self.app_state == AppState::Input
    }
}

pub fn init_app_and_terminal() -> (App, DefaultTerminal) {
    color_eyre::install().expect("error color_eyre");

    let app = App::default();

    (app, ratatui::init())
}

pub fn init_thread(tx: mpsc::Sender<AppEvent>) {
    thread::spawn(move || {
        loop {
            match event::read().unwrap() {
                Event::Key(key) => {
                    tx.send(AppEvent::Key(key)).unwrap();
                }

                Event::Paste(..) => {
                    tx.send(AppEvent::Paste()).unwrap();
                }
                _ => (),
            }
        }
    });
}

fn check_search_engine(app: &mut App) {
    let chars: Vec<char> = app.input_queue.chars().take(2).collect();

    match chars.as_slice() {
        ['!', 'n' | 'N' | 'Т' | 'т'] => {
            app.app_status = AppStatus::NixOS;
        }
        ['!', 't' | 'T' | 'Е' | 'е'] => {
            app.app_status = AppStatus::Translate;
        }

        _ => {
            app.app_status = AppStatus::DuckDuckGo;
        }
    }
}
