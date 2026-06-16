pub struct Fish {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
}

impl Fish {
    pub fn new(x: f32, y: f32, vx: f32) -> Self {
        Self { x, y, vx }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
    }

    pub fn sprite(&self) -> &'static str {
        if self.vx >= 0.0 {
            "     __\n ___( o)>\n \\ <_. )\n   '---`"
        } else {
            "   __\n <(o )___\n  ( ._> /\n   `---'"
        }
    }
}
