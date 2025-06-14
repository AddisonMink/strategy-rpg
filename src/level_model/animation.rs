use crate::{engine::*, level_model::Entity};

const ATTACK_DURATION: f32 = 0.1;
const DEATH_DURATION: f32 = 0.5;
const TEXT_DURATION: f32 = 0.5;
const PANEL_TEXT_DURATION: f32 = 0.5;

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
    PanelText {
        coord: Coord,
        text: String,
    },
    Entity(EntityAnimation),
}

#[derive(Debug, Clone)]
pub struct EntityAnimation {
    pub entity: Entity,
    pub kind: EntityAnimationKind,
}

#[derive(Debug, Clone, Copy)]
pub enum EntityAnimationKind {
    Attack { direction: Direction },
    Death,
}

impl Animation {
    pub fn text(coord: Coord, text: String, color: Color) -> Self {
        Animation {
            timer: Timer::new(TEXT_DURATION),
            kind: AnimationKind::Text { coord, text, color },
        }
    }

    pub fn panel_text(coord: Coord, text: String) -> Self {
        Animation {
            timer: Timer::new(PANEL_TEXT_DURATION),
            kind: AnimationKind::PanelText { coord, text },
        }
    }

    pub fn attack(entity: Entity, direction: Direction) -> Self {
        Animation {
            timer: Timer::new(ATTACK_DURATION),
            kind: AnimationKind::Entity(EntityAnimation {
                entity,
                kind: EntityAnimationKind::Attack { direction },
            }),
        }
    }

    pub fn death(entity: Entity) -> Self {
        Animation {
            timer: Timer::new(DEATH_DURATION),
            kind: AnimationKind::Entity(EntityAnimation {
                entity,
                kind: EntityAnimationKind::Death,
            }),
        }
    }

    pub fn animating_entity(&self) -> Option<Entity> {
        if let AnimationKind::Entity(EntityAnimation { entity, .. }) = &self.kind {
            Some(*entity)
        } else {
            None
        }
    }
}
