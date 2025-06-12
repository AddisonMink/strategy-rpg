use crate::engine::*;

const TEXT_DURATION: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct Animation {
    pub timer: Timer,
    pub kind: AnimationKind,
}

#[derive(Debug, Clone)]
pub enum AnimationKind {
    Text {
        coord: Coord,
        text: String,
        color: Color,
    },
}

impl Animation {
    pub fn text(coord: Coord, text: String, color: Color) -> Self {
        Animation {
            timer: Timer::new(TEXT_DURATION),
            kind: AnimationKind::Text { coord, text, color },
        }
    }
}
