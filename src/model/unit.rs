use std::collections::VecDeque;

use super::{Game, light::Light};
use crate::util::*;

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

    // state
    pub coord: Coord,
    pub light: Option<Light>,

    // Optional NPC behavior.
    pub npc_behavior: Option<NpcBehavior>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NpcBehavior {
    pub select_move: fn(&Unit, &Game) -> Option<VecDeque<Coord>>,
}
