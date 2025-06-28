use super::*;
use crate::constants::TILE_SIZE;
use crate::engine_v2::*;
use crate::util::*;

const TEXT_DURATION: f32 = 0.5;
const FADING_RISING_TEXT_MAX_OFFSET: f32 = TILE_SIZE / 2.0;
const ATTACK_DURATION: f32 = 0.1;
const DEATH_DURATION: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
pub enum UnitAnimationKind {
    Attack(Direction),
    Death,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitAnimation {
    pub id: UnitId,
    pub kind: UnitAnimationKind,
}

#[derive(Debug, Clone, Copy)]
pub enum AnimationKind {
    Sleep,
    Text(Coord, ShortString, Color),
    FadingRisingText(Coord, ShortString, Color, f32), // f32 for max offset
    UnitAnimation(UnitAnimation),
}

#[derive(Debug, Clone, Copy)]
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

    pub fn fading_rising_text(coord: Coord, text: ShortString, color: Color) -> Self {
        Self {
            timer: Timer::new(TEXT_DURATION),
            kind: AnimationKind::FadingRisingText(
                coord,
                text,
                color,
                FADING_RISING_TEXT_MAX_OFFSET,
            ),
        }
    }

    pub fn attack(id: UnitId, direction: Direction) -> Self {
        Self {
            timer: Timer::new(ATTACK_DURATION),
            kind: AnimationKind::UnitAnimation(UnitAnimation {
                id,
                kind: UnitAnimationKind::Attack(direction),
            }),
        }
    }

    pub fn death(id: UnitId) -> Self {
        Self {
            timer: Timer::new(DEATH_DURATION),
            kind: AnimationKind::UnitAnimation(UnitAnimation {
                id,
                kind: UnitAnimationKind::Death,
            }),
        }
    }

    pub fn unit_id(&self) -> Option<UnitId> {
        if let AnimationKind::UnitAnimation(UnitAnimation { id, .. }) = self.kind {
            Some(id)
        } else {
            None
        }
    }
}
