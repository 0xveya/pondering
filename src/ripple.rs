pub enum RippleKind {
    Wake { size: u8 },
    Ambient,
}

pub struct Ripple {
    pub kind: RippleKind,
    pub position: [f32; 2],
    pub age: f32,
}

impl Ripple {
    pub fn new(kind: RippleKind, position: [f32; 2]) -> Self {
        Self {
            kind,
            position,
            age: 0.0,
        }
    }

    pub fn phase(&self) -> usize {
        ((self.age / 2.0 * 3.0) as usize).min(2)
    }
}
