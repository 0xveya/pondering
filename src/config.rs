use crate::text::TextSize;
use crate::theme::Theme;

pub struct Config {
    pub ducks: usize,
    pub swarms: usize,
    pub clock: bool,
    pub endless: bool,
    pub quote: bool,
    pub theme: Theme,
    pub clock_size: TextSize,
}

impl Config {
    pub fn new(ducks: usize, swarms: usize, clock: bool, endless: bool, quote: bool) -> Self {
        Self {
            ducks,
            swarms,
            clock,
            endless,
            quote,
            theme: Theme::default(),
            clock_size: TextSize::Pixel(2),
        }
    }
}
