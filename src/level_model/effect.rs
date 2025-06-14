use super::entity::Entity;
use crate::{engine::*, level_model::Animation};

pub enum Effect {
    UpdateLightGrid,
    UpdateVisionGrid,
    UpdateAllNpcVision,
    UpdateAllNpcVisionOfPlayer { player: Entity },
    UpdateNpcVisionOfAllPlayers { npc: Entity },
    Move { entity: Entity, coord: Coord },
    Sleep { duration: f32 },
    Damage { entity: Entity, min: u16, max: u16 },
    Death { entity: Entity },
    Animation { animation: Animation },
}
