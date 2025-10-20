use std::sync::mpsc;

use color_eyre::eyre::Result;

mod app;
use app::init_app_and_terminal;

use crate::app::{AppEvent, init_thread};

fn main() -> Result<()> {
    let (app, terminal) = init_app_and_terminal();

    let (tx, rc) = mpsc::channel::<AppEvent>();

    init_thread(tx);

    let result = app.run(terminal, rc);

    ratatui::restore();
    result
}
