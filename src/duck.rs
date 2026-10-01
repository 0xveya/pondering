#[derive(Clone, Copy)]
pub enum DuckKind {
    Mallard,
    White,
    Duckling,
}

pub trait Behavior {
    fn velocity(
        &mut self,
        index: usize,
        flock: &[[f32; 2]],
        dt: f32,
        bounds: Option<[f32; 2]>,
    ) -> [f32; 2];
}

pub struct Wander {
    speed: f32,
    heading: f32,
    target_heading: f32,
    turn_in: f32,
}

impl Wander {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            heading: 0.0,
            target_heading: 0.0,
            turn_in: 0.0,
        }
    }
}

impl Behavior for Wander {
    fn velocity(
        &mut self,
        index: usize,
        flock: &[[f32; 2]],
        dt: f32,
        bounds: Option<[f32; 2]>,
    ) -> [f32; 2] {
        self.turn_in -= dt;

        if self.turn_in <= 0.0 {
            let angle = rand::random_range(-0.2..=0.2);
            self.target_heading = if self.heading.cos() < 0.0 {
                std::f32::consts::PI - angle
            } else {
                angle
            };
            self.turn_in = rand::random_range(1.0..=3.0);
        }

        let turn = (self.target_heading - self.heading + std::f32::consts::PI)
            .rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;
        self.heading += turn * (dt * 2.0).min(1.0);

        let mut velocity = [
            self.heading.cos() * self.speed,
            self.heading.sin() * self.speed,
        ];
        if let Some(bounds) = bounds {
            let position = flock[index];
            for axis in 0..2 {
                let next = position[axis] + velocity[axis] * dt;
                if bounds[axis] == 0.0 {
                    velocity[axis] = 0.0;
                } else if (next <= 0.0 && velocity[axis] < 0.0)
                    || (next >= bounds[axis] && velocity[axis] > 0.0)
                {
                    velocity[axis] = -velocity[axis];
                    self.heading = velocity[1].atan2(velocity[0]);
                    self.target_heading = self.heading;
                }
            }
        }
        velocity
    }
}

pub struct Follow {
    pub target: usize,
    pub speed: f32,
    pub distance: f32,
    pub separation: f32,
}

impl Behavior for Follow {
    fn velocity(
        &mut self,
        index: usize,
        flock: &[[f32; 2]],
        dt: f32,
        _bounds: Option<[f32; 2]>,
    ) -> [f32; 2] {
        if dt <= 0.0 {
            return [0.0, 0.0];
        }

        let Some(position) = flock.get(index) else {
            return [0.0, 0.0];
        };
        let Some(target) = flock.get(self.target) else {
            return [0.0, 0.0];
        };

        let mut velocity = [0.0, 0.0];

        let dx = target[0] - position[0];
        let dy = target[1] - position[1];
        let length = dx.hypot(dy);
        let remaining = length - self.distance;

        if remaining > 0.0 && length > 0.0 {
            let speed = self.speed.min(remaining / dt);
            velocity = [dx / length * speed, dy / length * speed];
        }

        for (other_index, other) in flock.iter().enumerate() {
            if other_index == index {
                continue;
            }

            let dx = position[0] - other[0];
            let dy = position[1] - other[1];
            let length = dx.hypot(dy);

            if length >= self.separation {
                continue;
            }

            if length == 0.0 {
                let direction = if index < other_index { -1.0 } else { 1.0 };
                velocity[0] += direction * self.speed;
                continue;
            }

            let strength = (1.0 - length / self.separation) * self.speed;
            velocity[0] += dx / length * strength;
            velocity[1] += dy / length * strength;
        }

        let speed = velocity[0].hypot(velocity[1]);
        if speed > self.speed {
            velocity[0] *= self.speed / speed;
            velocity[1] *= self.speed / speed;
        }

        velocity
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

    pub fn update(&mut self, index: usize, flock: &[[f32; 2]], dt: f32, bounds: Option<[f32; 2]>) {
        let velocity = self.behavior.velocity(index, flock, dt, bounds);

        if velocity[0] != 0.0 {
            self.facing_left = velocity[0] < 0.0;
        }

        self.position[0] += velocity[0] * dt;
        self.position[1] += velocity[1] * dt;
        if let Some(bounds) = bounds {
            self.position[0] = self.position[0].clamp(0.0, bounds[0]);
            self.position[1] = self.position[1].clamp(0.0, bounds[1]);
        }
    }
}
