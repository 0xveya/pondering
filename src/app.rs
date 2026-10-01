use crate::duck::{Duck, DuckKind, Follow, Wander};
use crate::theme::Theme;

pub struct App {
    pub ducks: Vec<Duck>,
    pub theme: Theme,
}

impl App {
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
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
        }
    }

    pub fn update(&mut self, dt: f32) {
        let positions: Vec<_> = self.ducks.iter().map(|duck| duck.position).collect();
        for (index, duck) in self.ducks.iter_mut().enumerate() {
            duck.update(index, &positions, dt);
        }
    }
}
