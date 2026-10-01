//! Duck pixel grids and palettes. Palette index zero is transparent.

use ratatui::style::Color;

use crate::duck::DuckKind;

pub struct Sprite {
    pub pixels: &'static [[u8; 12]],
    pub palette: [Color; 6],
}

const ADULT: [[u8; 12]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 3, 3],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
    [4, 0, 4, 4, 4, 4, 4, 4, 1, 0, 0, 0],
    [0, 4, 4, 5, 5, 5, 4, 4, 4, 0, 0, 0],
    [0, 4, 4, 4, 5, 5, 5, 4, 4, 0, 0, 0],
    [0, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0],
    [0, 0, 0, 4, 4, 4, 4, 0, 0, 0, 0, 0],
];

static MALLARD: Sprite = Sprite {
    pixels: &ADULT,
    palette: [
        Color::Reset,
        Color::Rgb(34, 145, 86),
        Color::Black,
        Color::Rgb(255, 185, 45),
        Color::Rgb(165, 125, 85),
        Color::Rgb(95, 70, 50),
    ],
};

static WHITE: Sprite = Sprite {
    pixels: &ADULT,
    palette: [
        Color::Reset,
        Color::Rgb(255, 250, 235),
        Color::Black,
        Color::Rgb(255, 155, 35),
        Color::Rgb(255, 250, 235),
        Color::Rgb(205, 205, 190),
    ],
};

static DUCKLING: Sprite = Sprite {
    pixels: &[
        [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 2, 3, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        [4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0],
        [0, 4, 5, 5, 4, 4, 0, 0, 0, 0, 0, 0],
        [0, 0, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0],
    ],
    palette: [
        Color::Reset,
        Color::Rgb(255, 225, 90),
        Color::Black,
        Color::Rgb(255, 155, 35),
        Color::Rgb(255, 225, 90),
        Color::Rgb(220, 175, 50),
    ],
};

pub fn get(kind: DuckKind) -> &'static Sprite {
    match kind {
        DuckKind::Mallard => &MALLARD,
        DuckKind::White => &WHITE,
        DuckKind::Duckling => &DUCKLING,
    }
}
