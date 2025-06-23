use crate::util::Timer;

pub struct Animation {
    pub timer: Timer,
    pub kind: AnimationKind,
}

impl Animation {
    pub fn sleep(duration: f32) -> Self {
        Self {
            timer: Timer::new(duration),
            kind: AnimationKind::Sleep,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AnimationKind {
    Sleep,
}
