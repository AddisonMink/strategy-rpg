use std::collections::HashSet;
use std::vec;

use super::Action;
use super::Battle;
use super::Effect;
use crate::engine::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    // Identifiers
    pub id: UnitId,
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    // Attributes
    pub movement: u16,
    pub hp_max: u16,
    pub vision: u16,
    pub tags: HashSet<UnitTag>,
    // State
    pub coord: Coord,
    pub hp: u16,
    // NPC-Specific Fields
    pub last_seen_player: Option<(UnitId, Coord)>,
    pub select_move: fn(&Battle) -> VecDeque<Coord>,
    pub select_action: fn(&Battle) -> VecDeque<Effect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    NPC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitTag {
    Lurker,
}

pub struct UnitData {
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    pub movement: u16,
    pub vision: u16,
    pub hp_max: u16,
    pub tags: ShortList<UnitTag>,
}

impl Unit {
    pub fn new(
        id: UnitId,
        coord: Coord,
        data: UnitData,
        select_move: Option<fn(&Battle) -> VecDeque<Coord>>,
        select_action: Option<fn(&Battle) -> VecDeque<Effect>>,
    ) -> Self {
        Self {
            id,
            name: data.name,
            glyph: data.glyph,
            side: data.side,
            movement: data.movement,
            vision: data.vision,
            hp_max: data.hp_max,
            tags: HashSet::from_iter(data.tags.as_slice().iter().cloned()),
            coord,
            hp: data.hp_max,
            last_seen_player: None,
            select_move: select_move.unwrap_or_else(|| |_: &Battle| VecDeque::new()),
            select_action: select_action.unwrap_or_else(|| |_: &Battle| VecDeque::new()),
        }
    }

    pub fn actions(&self) -> Vec<Action> {
        vec![Action::ATTACK]
    }
}
