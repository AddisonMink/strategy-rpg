use super::*;
use crate::engine::{Direction, Timer};

const BOUNCE_DURATION: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
pub struct UnitAnimation {
    pub timer: Timer,
}

#[derive(Debug, Clone, Copy)]
pub enum UnitAnimationKind {
    Bounce { direction: Direction },
}

impl UnitAnimation {
    pub fn bounce(direction: Direction) -> Self {
        UnitAnimation {
            timer: Timer::new(BOUNCE_DURATION),
        }
    }
}
