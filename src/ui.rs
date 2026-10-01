use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::{app::App, sprites, theme};

pub fn draw(frame: &mut Frame, app: &App) {
    let inside = frame.area();
    let style = Style::default().fg(theme::TEXT).bg(theme::WATER);
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

    for duck in &app.ducks {
        let sprite = sprites::get(duck.kind);
        for (row, pair) in sprite.pixels.chunks(2).enumerate() {
            for col in 0..12 {
                let source_col = if duck.facing_left { 11 - col } else { col };
                let top = pair[0][source_col];
                let bottom = pair.get(1).map_or(0, |pixels| pixels[source_col]);
                if top == 0 && bottom == 0 {
                    continue;
                }

                let x = i32::from(inside.x) + duck.position[0].floor() as i32 + col as i32;
                let y = i32::from(inside.y) + duck.position[1].floor() as i32 + row as i32;
                if x < i32::from(inside.x)
                    || y < i32::from(inside.y)
                    || x >= i32::from(inside.right())
                    || y >= i32::from(inside.bottom())
                {
                    continue;
                }

                if let Some(cell) = frame.buffer_mut().cell_mut((x as u16, y as u16)) {
                    let underneath = cell.bg;
                    let foreground = if top == 0 {
                        underneath
                    } else {
                        sprite.palette[top as usize]
                    };
                    let background = if bottom == 0 {
                        underneath
                    } else {
                        sprite.palette[bottom as usize]
                    };
                    cell.set_char('▀').set_fg(foreground).set_bg(background);
                }
            }
        }
    }
}
