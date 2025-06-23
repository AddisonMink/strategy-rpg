#[derive(Debug, Clone, Copy)]
pub struct Timer {
    elapsed: f32,
    duration: f32,
}

impl Timer {
    pub fn new(duration: f32) -> Self {
        Timer {
            elapsed: 0.0,
            duration,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }

    pub fn progress(&self) -> f32 {
        if self.duration == 0.0 {
            1.0
        } else {
            (self.elapsed / self.duration).min(1.0)
        }
    }
}
