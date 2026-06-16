use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::{app::App, theme};

pub fn draw(frame: &mut Frame, app: &App) {
    let inside = frame.area();
    let style = Style::default().fg(theme::FISH).bg(theme::WATER);
    frame.render_widget(Block::default().style(style), inside);

    if inside.height > 0 {
        let width = inside.width.min(14);
        let message = Rect::new(
            inside.x + (inside.width - width) / 2,
            inside.y + inside.height / 2,
            width,
            1,
        );
        frame.render_widget(Paragraph::new("MUCH TO PONDER").style(style), message);
    }

    for fish in &app.fish {
        if fish.x < 0.0 || fish.y < 0.0 {
            continue;
        }

        let x = fish.x as u16;
        let y = fish.y as u16;
        if x >= inside.width || y >= inside.height {
            continue;
        }

        let area = Rect::new(
            inside.x + x,
            inside.y + y,
            inside.width - x,
            (inside.height - y).min(4),
        );
        frame.render_widget(Paragraph::new(fish.sprite()).style(style), area);
    }
}
