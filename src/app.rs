use ratatui::layout::Size;

use crate::duck::{Duck, DuckKind, Follow, Wander};
use crate::ripple::{Ripple, RippleKind};
use crate::sprites;
use crate::text::TextSize;
use crate::theme::Theme;

pub struct App {
    pub ducks: Vec<Duck>,
    pub theme: Theme,
    pub clock_size: TextSize,
    pub ripples: Vec<Ripple>,
    pub elapsed: f32,
    wake_in: f32,
    ambient_in: f32,
}

impl App {
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
            clock_size: TextSize::Pixel(2),
            ducks: vec![
                Duck::new(DuckKind::Mallard, [24.0, 7.0], Wander::new(3.0)),
                Duck::new(
                    DuckKind::White,
                    [12.0, 9.0],
                    Follow {
                        target: 0,
                        speed: 4.0,
                        distance: 14.0,
                        separation: 12.0,
                    },
                ),
                Duck::new(
                    DuckKind::Duckling,
                    [3.0, 7.0],
                    Follow {
                        target: 0,
                        speed: 4.0,
                        distance: 16.0,
                        separation: 12.0,
                    },
                ),
            ],
            ripples: Vec::new(),
            elapsed: 0.0,
            wake_in: 0.0,
            ambient_in: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, size: Size) {
        self.elapsed += dt;
        for ripple in &mut self.ripples {
            ripple.age += dt;
        }
        self.ripples.retain(|ripple| ripple.age < 2.0);

        self.wake_in -= dt;
        let emit_wakes = self.wake_in <= 0.0;

        if emit_wakes {
            self.wake_in = 0.45;
        }

        let positions: Vec<_> = self.ducks.iter().map(|duck| duck.position).collect();

        for (index, duck) in self.ducks.iter_mut().enumerate() {
            duck.update(index, &positions, dt);

            if emit_wakes && duck.position != positions[index] {
                let sprite = sprites::get(duck.kind);
                let height = sprite.pixels.len().div_ceil(2);

                let tail_x = if duck.facing_left { 12.0 } else { -1.0 };
                let position = [
                    duck.position[0] + tail_x,
                    duck.position[1] + height as f32 - 1.0,
                ];

                self.ripples.push(Ripple::new(
                    RippleKind::Wake {
                        size: (height / 2) as u8,
                    },
                    position,
                ));
            }
        }

        self.ambient_in -= dt;

        if self.ambient_in <= 0.0 && size.width > 0 && size.height > 0 {
            let position = [
                f32::from(rand::random_range(0..size.width)),
                f32::from(rand::random_range(0..size.height)),
            ];

            self.ripples
                .push(Ripple::new(RippleKind::Ambient, position));
            self.ambient_in = rand::random_range(0.4..=1.0);
        }
    }
}
