use ratatui::style::Color;

use crate::duck::DuckKind;

#[derive(Clone, Copy)]
pub struct DuckPalette {
    pub head: Color,
    pub eye: Color,
    pub beak: Color,
    pub body: Color,
    pub wing: Color,
}

pub struct Theme {
    pub water: Color,
    pub text: Color,
    pub mallard: DuckPalette,
    pub white: DuckPalette,
    pub duckling: DuckPalette,
}

impl Theme {
    pub fn palette(&self, kind: DuckKind) -> [Color; 6] {
        let palette = match kind {
            DuckKind::Mallard => self.mallard,
            DuckKind::White => self.white,
            DuckKind::Duckling => self.duckling,
        };
        [
            Color::Reset,
            palette.head,
            palette.eye,
            palette.beak,
            palette.body,
            palette.wing,
        ]
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            water: Color::Rgb(32, 60, 85),
            text: Color::Rgb(235, 226, 193),
            mallard: DuckPalette {
                head: Color::Rgb(34, 145, 86),
                eye: Color::Black,
                beak: Color::Rgb(255, 185, 45),
                body: Color::Rgb(165, 125, 85),
                wing: Color::Rgb(95, 70, 50),
            },
            white: DuckPalette {
                head: Color::Rgb(255, 250, 235),
                eye: Color::Black,
                beak: Color::Rgb(255, 155, 35),
                body: Color::Rgb(255, 250, 235),
                wing: Color::Rgb(205, 205, 190),
            },
            duckling: DuckPalette {
                head: Color::Rgb(255, 225, 90),
                eye: Color::Black,
                beak: Color::Rgb(255, 155, 35),
                body: Color::Rgb(255, 225, 90),
                wing: Color::Rgb(220, 175, 50),
            },
        }
    }
}
