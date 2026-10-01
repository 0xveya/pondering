mod app;
mod config;
mod duck;
mod ripple;
mod sprites;
mod text;
mod theme;
mod ui;

use std::time::{Duration, Instant};

use clap::Parser;

use app::App;
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use text::TextSize;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 3, help = "Ducks per swarm")]
    ducks: u16,

    #[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u16).range(1..), help = "Number of swarms")]
    swarms: u16,

    #[arg(long, help = "Show the local clock")]
    clock: bool,

    #[arg(long, help = "Keep swarms inside the terminal")]
    endless: bool,

    #[arg(long, help = "Hide the quote")]
    no_quote: bool,

    #[arg(
        long,
        default_value_t = 2,
        help = "Clock pixel scale; 0 uses normal text"
    )]
    clock_size: u8,
}

fn convert_args(args: Args) -> config::Config {
    let mut config = config::Config::new(
        usize::from(args.ducks),
        usize::from(args.swarms),
        args.clock,
        args.endless,
        !args.no_quote,
    );
    config.clock_size = if args.clock_size == 0 {
        TextSize::Normal
    } else {
        TextSize::Pixel(args.clock_size)
    };
    config
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();
    let cfg = convert_args(args);

    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, cfg);
    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal, cfg: config::Config) -> color_eyre::Result<()> {
    let mut app = App::new(cfg, terminal.size()?);

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
            let size = terminal.size()?;
            app.update(last_tick.elapsed().as_secs_f32(), size);
            last_tick = Instant::now();
            if app.finished(size) {
                break;
            }
        }
    }

    Ok(())
}
