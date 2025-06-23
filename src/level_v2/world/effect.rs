use crate::{level_v2::world::UnitId, util::Coord};

#[derive(Debug, Clone, Copy)]
pub enum Effect {
    UpdateLightGrid,
    UpdatePlayerVision,
    UpdateNpcVision,
    Sleep { duration: f32 },
    Move { id: UnitId, coord: Coord },
}
