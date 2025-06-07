use super::*;
use crate::engine::*;

const NUMBER_DURATION: f32 = 0.5;
const ATTACK_DURATION: f32 = 0.125;

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
    Attack {
        unit_id: UnitId,
        direction: Direction,
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

    pub fn attack(unit_id: UnitId, direction: Direction) -> Self {
        Self {
            timer: Timer::new(ATTACK_DURATION),
            kind: AnimationKind::Attack { unit_id, direction },
        }
    }
}
