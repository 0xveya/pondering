#[derive(Clone, Copy)]
pub enum DuckKind {
    Mallard,
    White,
    Duckling,
}

pub trait Behavior {
    fn velocity(&mut self, position: [f32; 2], flock: &[[f32; 2]], dt: f32) -> [f32; 2];
}

pub struct Wander {
    pub speed: f32,
    pub heading: f32,
}

impl Behavior for Wander {
    fn velocity(&mut self, _position: [f32; 2], _flock: &[[f32; 2]], dt: f32) -> [f32; 2] {
        self.heading += dt * 0.5;

        [
            self.heading.cos() * self.speed,
            self.heading.sin() * self.speed,
        ]
    }
}

pub struct Follow {
    pub target: usize,
    pub speed: f32,
    pub distance: f32,
}

impl Behavior for Follow {
    fn velocity(&mut self, position: [f32; 2], flock: &[[f32; 2]], dt: f32) -> [f32; 2] {
        let Some(target) = flock.get(self.target) else {
            return [0.0, 0.0];
        };

        let dx = target[0] - position[0];
        let dy = target[1] - position[1];
        let length = dx.hypot(dy);
        let remaining = length - self.distance;

        if remaining <= 0.0 || length == 0.0 || dt <= 0.0 {
            return [0.0, 0.0];
        }

        let speed = self.speed.min(remaining / dt);
        [dx / length * speed, dy / length * speed]
    }
}

pub struct Duck {
    pub kind: DuckKind,
    pub position: [f32; 2],
    pub facing_left: bool,
    behavior: Box<dyn Behavior>,
}

impl Duck {
    pub fn new(kind: DuckKind, position: [f32; 2], behavior: impl Behavior + 'static) -> Self {
        Self {
            kind,
            position,
            facing_left: false,
            behavior: Box::new(behavior),
        }
    }

    pub fn update(&mut self, flock: &[[f32; 2]], dt: f32) {
        let velocity = self.behavior.velocity(self.position, flock, dt);

        if velocity[0] != 0.0 {
            self.facing_left = velocity[0] < 0.0;
        }

        self.position[0] += velocity[0] * dt;
        self.position[1] += velocity[1] * dt;
    }
}
