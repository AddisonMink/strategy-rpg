use crate::engine::*;

const NUMBER_DURATION: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
pub struct Animation {
    pub timer: Timer,
    pub kind: AnimationKind,
}

#[derive(Debug, Clone, Copy)]
pub enum AnimationKind {
    Number {
        coord: Coord,
        value: i32,
        color: Color,
    },
}

impl Animation {
    pub fn number(coord: Coord, value: i32, color: Color) -> Self {
        Self {
            timer: Timer::new(NUMBER_DURATION),
            kind: AnimationKind::Number {
                coord,
                value,
                color,
            },
        }
    }
}
