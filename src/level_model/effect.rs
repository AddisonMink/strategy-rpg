use super::entity::Entity;
use crate::engine::*;

pub enum Effect {
    UpdateLightGrid,
    UpdateVisionGrid,
    Move { entity: Entity, coord: Coord },
    Sleep { duration: f32 },
}
