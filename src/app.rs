use crate::fish::Fish;

pub struct App {
    pub fish: Vec<Fish>,
}

impl App {
    pub fn new() -> Self {
        Self {
            fish: vec![Fish::new(10.0, 5.0, 6.0), Fish::new(30.0, 10.0, -4.0)],
        }
    }

    pub fn update(&mut self, dt: f32) {
        for fish in &mut self.fish {
            fish.update(dt);
        }
    }
}
