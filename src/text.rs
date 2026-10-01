use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::Paragraph,
};

#[derive(Clone, Copy)]
pub enum TextSize {
    Normal,
    Pixel(u8),
}

pub struct TextLine<'a> {
    pub text: &'a str,
    pub size: TextSize,
}

pub fn draw_centered_lines(frame: &mut Frame, area: Rect, lines: &[TextLine<'_>], style: Style) {
    let mut rows = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            rows.push(String::new());
        }
        rows.extend(render(line.text, line.size, area));
    }

    if rows.len() > usize::from(area.height) {
        rows = lines.iter().map(|line| line.text.to_owned()).collect();
    }

    let height = rows.len().min(usize::from(area.height)) as u16;
    let top = area.y + (area.height - height) / 2;
    let width = rows
        .iter()
        .map(|row| Line::from(row.as_str()).width())
        .max()
        .unwrap_or(0);
    let width = width.min(usize::from(area.width)) as u16;
    let left = area.x + (area.width - width) / 2;

    for (row, text) in rows.iter().take(usize::from(height)).enumerate() {
        frame.render_widget(
            Paragraph::new(text.as_str())
                .alignment(Alignment::Center)
                .style(style),
            Rect::new(left, top + row as u16, width, 1),
        );
    }
}

fn render(text: &str, size: TextSize, area: Rect) -> Vec<String> {
    let TextSize::Pixel(scale) = size else {
        return vec![text.to_owned()];
    };
    let scale = usize::from(scale);
    let width = text
        .chars()
        .count()
        .saturating_mul(4)
        .saturating_sub(1)
        .saturating_mul(scale);
    let height = (5 * scale).div_ceil(2);
    if scale == 0 || width > usize::from(area.width) || height > usize::from(area.height) {
        return vec![text.to_owned()];
    }
    let Some(glyphs): Option<Vec<_>> = text.chars().map(glyph).collect() else {
        return vec![text.to_owned()];
    };

    let mut rows = Vec::with_capacity(height);
    for row in (0..5 * scale).step_by(2) {
        let mut line = String::new();
        for (index, glyph) in glyphs.iter().enumerate() {
            if index > 0 {
                line.extend(std::iter::repeat_n(' ', scale));
            }
            for bit in (0..3).rev() {
                let top = glyph[row / scale] & (1 << bit) != 0;
                let bottom = glyph
                    .get((row + 1) / scale)
                    .is_some_and(|pixels| pixels & (1 << bit) != 0);
                let pixel = match (top, bottom) {
                    (false, false) => ' ',
                    (true, false) => '▀',
                    (false, true) => '▄',
                    (true, true) => '█',
                };
                line.extend(std::iter::repeat_n(pixel, scale));
            }
        }
        rows.push(line);
    }
    rows
}

fn glyph(character: char) -> Option<[u8; 5]> {
    Some(match character.to_ascii_uppercase() {
        '0' => [7, 5, 5, 5, 7],
        '1' => [2, 6, 2, 2, 7],
        '2' => [7, 1, 7, 4, 7],
        '3' => [7, 1, 7, 1, 7],
        '4' => [5, 5, 7, 1, 1],
        '5' => [7, 4, 7, 1, 7],
        '6' => [7, 4, 7, 5, 7],
        '7' => [7, 1, 1, 1, 1],
        '8' => [7, 5, 7, 5, 7],
        '9' => [7, 5, 7, 1, 7],
        'A' => [2, 5, 7, 5, 5],
        'B' => [6, 5, 6, 5, 6],
        'C' => [7, 4, 4, 4, 7],
        'D' => [6, 5, 5, 5, 6],
        'E' => [7, 4, 6, 4, 7],
        'F' => [7, 4, 6, 4, 4],
        'G' => [7, 4, 5, 5, 7],
        'H' => [5, 5, 7, 5, 5],
        'I' => [7, 2, 2, 2, 7],
        'J' => [1, 1, 1, 5, 7],
        'K' => [5, 5, 6, 5, 5],
        'L' => [4, 4, 4, 4, 7],
        'M' => [5, 7, 7, 5, 5],
        'N' => [5, 7, 7, 7, 5],
        'O' => [7, 5, 5, 5, 7],
        'P' => [7, 5, 7, 4, 4],
        'Q' => [7, 5, 5, 7, 1],
        'R' => [6, 5, 6, 5, 5],
        'S' => [7, 4, 7, 1, 7],
        'T' => [7, 2, 2, 2, 2],
        'U' => [5, 5, 5, 5, 7],
        'V' => [5, 5, 5, 5, 2],
        'W' => [5, 5, 7, 7, 5],
        'X' => [5, 5, 2, 5, 5],
        'Y' => [5, 5, 2, 2, 2],
        'Z' => [7, 1, 2, 4, 7],
        ':' => [0, 2, 0, 2, 0],
        '.' => [0, 0, 0, 0, 2],
        ',' => [0, 0, 0, 2, 4],
        '-' => [0, 0, 7, 0, 0],
        '!' => [2, 2, 2, 0, 2],
        '?' => [7, 1, 2, 0, 2],
        '\'' => [2, 2, 0, 0, 0],
        ' ' => [0; 5],
        _ => return None,
    })
}
