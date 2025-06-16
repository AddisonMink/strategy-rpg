use super::unit::UnitId;
use crate::{
    engine::*,
    level_model::{Animation, ItemId},
};

pub enum Effect {
    UpdateLightGrid,
    UpdateVisionGrid,
    UpdateAllNpcVision,
    UpdateAllNpcVisionOfPlayer {
        player: UnitId,
    },
    UpdateNpcVisionOfAllPlayers {
        npc: UnitId,
    },
    Move {
        entity: UnitId,
        coord: Coord,
    },
    Sleep {
        duration: f32,
    },
    Damage {
        entity: UnitId,
        min: u16,
        max: u16,
    },
    Death {
        entity: UnitId,
    },
    Animation {
        animation: Animation,
    },
    UseItem {
        entity: UnitId,
        item: ItemId,
        amount: u16,
    },
    BreakItem {
        entity: UnitId,
        item: ItemId,
    },
    AddLightToEntity {
        entity: UnitId,
        color: Color,
        radius: u16,
    },
}
