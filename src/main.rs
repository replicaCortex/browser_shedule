use std::{io::stdout, sync::mpsc};

use color_eyre::eyre::Result;

mod app;
use app::init_app_and_terminal;
use crossterm::event::{DisableBracketedPaste, EnableBracketedPaste};
use crossterm::execute;

use crate::app::{AppEvent, init_thread};

fn main() -> Result<()> {
    execute!(stdout(), EnableBracketedPaste).unwrap();

    let (app, terminal) = init_app_and_terminal();

    let (tx, rc) = mpsc::channel::<AppEvent>();

    init_thread(tx);

    let result = app.run(terminal, rc);

    ratatui::restore();

    execute!(stdout(), DisableBracketedPaste).unwrap();
    result
}
