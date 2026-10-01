use ratatui::layout::Size;

use crate::duck::{Duck, DuckKind, Follow, Wander};
use crate::ripple::{Ripple, RippleKind};
use crate::{config::Config, sprites};

pub struct App {
    pub ducks: Vec<Duck>,
    pub config: Config,
    pub ripples: Vec<Ripple>,
    pub elapsed: f32,
    wake_in: f32,
    ambient_in: f32,
}

impl App {
    pub fn new(cfg: Config, size: Size) -> Self {
        let mut ducks = Vec::new();
        let width = f32::from(size.width.saturating_sub(12));
        let height = f32::from(size.height.saturating_sub(4));
        for _ in 0..cfg.swarms {
            let origin = [
                rand::random_range(0.0..=width),
                rand::random_range(0.0..=height),
            ];
            for member in 0..cfg.ducks {
                let kind = match member % 3 {
                    0 => DuckKind::Mallard,
                    1 => DuckKind::White,
                    _ => DuckKind::Duckling,
                };
                let position = if member == 0 {
                    origin
                } else {
                    [
                        rand::random_range(
                            (origin[0] - 24.0).max(0.0)..=(origin[0] + 24.0).min(width),
                        ),
                        rand::random_range(
                            (origin[1] - 6.0).max(0.0)..=(origin[1] + 6.0).min(height),
                        ),
                    ]
                };
                let duck = if member == 0 {
                    Duck::new(kind, position, Wander::new(3.0))
                } else {
                    Duck::new(
                        kind,
                        position,
                        Follow {
                            target: 0,
                            speed: 4.0,
                            distance: 12.0 + member as f32 * 2.0,
                            separation: 12.0,
                        },
                    )
                };
                ducks.push(duck);
            }
        }
        Self {
            config: cfg,
            ducks,
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

        for swarm in self.ducks.chunks_mut(self.config.ducks.max(1)) {
            let positions: Vec<_> = swarm.iter().map(|duck| duck.position).collect();
            for (index, duck) in swarm.iter_mut().enumerate() {
                let sprite = sprites::get(duck.kind);
                let height = sprite.pixels.len().div_ceil(2);
                let bounds = self.config.endless.then_some([
                    f32::from(size.width.saturating_sub(12)),
                    f32::from(size.height.saturating_sub(height as u16)),
                ]);
                duck.update(index, &positions, dt, bounds);

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

    pub fn finished(&self, size: Size) -> bool {
        !self.config.endless
            && !self.ducks.is_empty()
            && self.ducks.iter().all(|duck| {
                let height = sprites::get(duck.kind).pixels.len().div_ceil(2) as f32;
                duck.position[0] >= f32::from(size.width)
                    || duck.position[0] + 12.0 <= 0.0
                    || duck.position[1] >= f32::from(size.height)
                    || duck.position[1] + height <= 0.0
            })
    }
}
