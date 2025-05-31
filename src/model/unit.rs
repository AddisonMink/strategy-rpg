use super::*;
use crate::util::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone)]
pub struct Unit {
    // identifiers
    pub id: UnitId,
    pub is_player: bool,
    pub glyph: Glyph,
    pub name: String,

    // attributes
    pub vision: u16,
    pub movement: u16,
    pub hp_max: u16,

    // state
    pub coord: Coord,
    pub hp: u16,
    pub light: Option<Light>,

    // Optional NPC behavior.
    pub npc_behavior: Option<NpcBehavior>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NpcBehavior {
    pub select_move: fn(&Unit, &Game) -> Option<VecDeque<Coord>>,
}
