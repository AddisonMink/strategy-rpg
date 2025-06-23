use crate::engine_v2::*;
use crate::util::*;

const TEXT_DURATION: f32 = 1.0;

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

    pub fn text(coord: Coord, text: ShortString, color: Color) -> Self {
        Self {
            timer: Timer::new(TEXT_DURATION),
            kind: AnimationKind::Text(coord, text, color),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AnimationKind {
    Sleep,
    Text(Coord, ShortString, Color),
}
