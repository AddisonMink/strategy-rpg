use super::entity::Entity;
use crate::engine::*;

pub enum Effect {
    UpdateLightGrid,
    UpdateVisionGrid,
    Move { entity: Entity, coord: Coord },
    Sleep { duration: f32 },
    Damage { entity: Entity, min: u16, max: u16 },
}
