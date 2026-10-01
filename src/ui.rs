use ratatui::{Frame, style::Style, widgets::Block};

use chrono::Local;

use crate::{
    app::App,
    ripple::RippleKind,
    sprites,
    text::{TextLine, TextSize, draw_centered_lines},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let inside = frame.area();
    let style = Style::default()
        .fg(app.config.theme.text)
        .bg(app.config.theme.water);
    frame.render_widget(Block::default().style(style), inside);

    for row in (0..inside.height).step_by(4) {
        for col in (0..inside.width).step_by(18) {
            let seed = u32::from(row) * 173 + u32::from(col) * 59;
            let phase = app.elapsed * 0.6 + seed as f32;
            let drift = (phase.sin() * 2.0).round() as i32;
            let x = i32::from(inside.x) + i32::from(col) + (seed % 11) as i32 + drift;
            let y = inside.y + row + (seed % 3) as u16;
            let brightness = ((phase.cos() + 1.0) as usize).min(2);
            let width = 2 + seed % 3;

            for offset in 0..width {
                let x = x + offset as i32;
                if x < i32::from(inside.x) || x >= i32::from(inside.right()) || y >= inside.bottom()
                {
                    continue;
                }
                if let Some(cell) = frame.buffer_mut().cell_mut((x as u16, y)) {
                    cell.set_char('▁')
                        .set_fg(app.config.theme.wavelets[brightness])
                        .set_bg(app.config.theme.water);
                }
            }
        }
    }

    for ripple in &app.ripples {
        let phase = ripple.phase();
        let (size, color_index) = match ripple.kind {
            RippleKind::Wake { size } => (i32::from(size), phase),
            RippleKind::Ambient => (1, (phase + 1).min(2)),
        };
        let radius = size + phase as i32;

        for offset in -radius..=radius {
            if (phase == 1 && offset.abs() < radius - 1) || (phase == 2 && offset.abs() < radius) {
                continue;
            }

            let x = i32::from(inside.x) + ripple.position[0].floor() as i32 + offset;
            let y = i32::from(inside.y) + ripple.position[1].floor() as i32;
            if x < i32::from(inside.x)
                || y < i32::from(inside.y)
                || x >= i32::from(inside.right())
                || y >= i32::from(inside.bottom())
            {
                continue;
            }

            if let Some(cell) = frame.buffer_mut().cell_mut((x as u16, y as u16)) {
                let symbol = if offset.abs() == radius { '▔' } else { '▁' };
                cell.set_char(symbol)
                    .set_fg(app.config.theme.ripples[color_index])
                    .set_bg(app.config.theme.water);
            }
        }
    }

    for duck in &app.ducks {
        let sprite = sprites::get(duck.kind);
        let palette = app.config.theme.palette(duck.kind);
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
                    let foreground = if top == 0 {
                        if cell.symbol() == "▀" {
                            cell.fg
                        } else {
                            cell.bg
                        }
                    } else {
                        palette[top as usize]
                    };
                    let background = if bottom == 0 {
                        cell.bg
                    } else {
                        palette[bottom as usize]
                    };
                    cell.set_char('▀').set_fg(foreground).set_bg(background);
                }
            }
        }
    }

    let time = Local::now().format("%H:%M").to_string();
    let quote = "tulln is giga kaka";

    let mut lines = Vec::with_capacity(3);
    if app.config.quote {
        lines.push(TextLine {
            text: quote,
            size: TextSize::Normal,
        });
    }
    if app.config.clock {
        lines.push(TextLine {
            text: time.as_str(),
            size: app.config.clock_size,
        });
    }
    lines.push(TextLine {
        text: "MUCH TO PONDER",
        size: TextSize::Normal,
    });
    draw_centered_lines(frame, inside, &lines, style);
}
