mod key_proccesing;

mod ui;

use std::sync::mpsc;
use std::thread;

use color_eyre::eyre::{Ok, Result};
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::{self, KeyEvent};
use ratatui::prelude::*;

pub enum AppEvent {
    Key(KeyEvent),
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
}

impl App {
    pub fn run(
        mut self,
        mut terminal: DefaultTerminal,
        rc: mpsc::Receiver<AppEvent>,
    ) -> Result<()> {
        while self.is_runing() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            match rc.recv().unwrap() {
                AppEvent::Key(key_event) => key_proccesing::handle_key_event(&mut self, key_event),
            }
        }

        Ok(())
    }

    fn is_runing(&self) -> bool {
        self.app_state == AppState::Normal || self.app_state == AppState::Input
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let vertical_chunks = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

        ui::render_tip(vertical_chunks[1], buf, self);
        ui::render_input(vertical_chunks[3], buf, self);
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
            if let Event::Key(key) = event::read().unwrap() {
                tx.send(AppEvent::Key(key)).unwrap();
            }
        }
    });
}
