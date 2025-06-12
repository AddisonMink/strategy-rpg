use crate::{engine::*, level_model::Entity};

const ATTACK_DURATION: f32 = 0.1;
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
}

impl Animation {
    pub fn text(coord: Coord, text: String, color: Color) -> Self {
        Animation {
            timer: Timer::new(TEXT_DURATION),
            kind: AnimationKind::Text { coord, text, color },
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

    pub fn animating_entity(&self) -> Option<Entity> {
        if let AnimationKind::Entity(EntityAnimation { entity, .. }) = &self.kind {
            Some(*entity)
        } else {
            None
        }
    }
}
