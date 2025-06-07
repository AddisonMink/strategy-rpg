use super::*;
use crate::engine::*;

const NUMBER_DURATION: f32 = 1.0;
const ATTACK_DURATION: f32 = 0.125;
const PANEL_MESSAGE_DURATION: f32 = 1.0;

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
    PanelMessage {
        coord: Coord,
        title: ShortString,
        title_color: Color,
        message: ShortString,
        message_color: Color,
    },
    Message {
        coord: Coord,
        text: ShortString,
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

    pub fn attack(unit_id: UnitId, direction: Direction) -> Self {
        Self {
            timer: Timer::new(ATTACK_DURATION),
            kind: AnimationKind::Attack { unit_id, direction },
        }
    }

    pub fn panel_message(
        coord: Coord,
        title: ShortString,
        title_color: Color,
        message: ShortString,
        message_color: Color,
    ) -> Self {
        Self {
            timer: Timer::new(PANEL_MESSAGE_DURATION),
            kind: AnimationKind::PanelMessage {
                coord,
                title,
                title_color,
                message,
                message_color,
            },
        }
    }

    pub fn action_message(unit: &Unit, action_name: ShortString, action_color: Color) -> Self {
        Self::panel_message(
            unit.coord,
            unit.name,
            unit.glyph.color,
            action_name,
            action_color,
        )
    }

    pub fn message(coord: Coord, text: ShortString, color: Color) -> Self {
        Self {
            timer: Timer::new(PANEL_MESSAGE_DURATION),
            kind: AnimationKind::Message { coord, text, color },
        }
    }
}
