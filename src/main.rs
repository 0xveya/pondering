mod app;
mod fish;
mod theme;
mod ui;

use std::time::{Duration, Instant};

use app::App;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut app = App::new();

    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.code == KeyCode::Char('q')
        {
            break;
        }

        if last_tick.elapsed() >= tick_rate {
            app.update(last_tick.elapsed().as_secs_f32());
            last_tick = Instant::now();
        }
    }

    Ok(())
}
