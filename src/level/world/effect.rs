use super::*;
use crate::{level::world::UnitId, util::Coord};

#[derive(Debug, Clone, Copy)]
pub enum Effect {
    UpdateLightGrid,
    UpdatePlayerVision,
    UpdateNpcVision,
    Sleep {
        duration: f32,
    },
    Move {
        id: UnitId,
        coord: Coord,
    },
    Damage {
        id: UnitId,
        min: u16,
        max: u16,
    },
    Kill {
        id: UnitId,
    },
    ConsumeCharge {
        id: UnitId,
        item_id: ItemId,
        amount: u16,
    },
    AddUnitLight {
        id: UnitId,
        light: Light,
    },
    Animate {
        animation: Animation,
    },
}
