use super::Effect;
use super::Level;
use super::item::*;
use super::light::*;
use crate::engine::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone)]
pub struct Unit {
    // Identifiers
    pub entity: UnitId,
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    // Attributes
    pub vision: u16,
    pub movement: u16,
    pub hp_max: u16,
    pub tags: HashSet<UnitTag>,
    // State
    pub hp: u16,
    pub coord: Coord,
    pub light: Option<Light>,
    // Player Fields
    pub items: HashMap<ItemId, Item>,
    // NPC Fields
    pub memory: Memory,
    pub behavior: Behavior,
}

impl Unit {
    pub fn new(
        entity: UnitId,
        name: ShortString,
        glyph: Glyph,
        side: Side,
        vision: u16,
        movement: u16,
        hp_max: u16,
        coord: Coord,
        behavior_opt: Option<Behavior>,
    ) -> Self {
        Self {
            entity,
            name,
            glyph,
            side,
            vision,
            movement,
            hp_max,
            tags: HashSet::new(),
            hp: hp_max,
            coord,
            light: None,
            items: HashMap::new(),
            memory: Memory::default(),
            behavior: behavior_opt.unwrap_or_default(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.id, item);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    NPC,
}

#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub visible_players: HashSet<UnitId>,
    pub last_seen_player: Option<(UnitId, Coord)>,
}

#[derive(Debug, Clone)]
pub struct Behavior {
    pub select_move: fn(&Level) -> Option<VecDeque<Coord>>,
    pub select_action: fn(&Level) -> Option<VecDeque<Effect>>,
}

impl Default for Behavior {
    fn default() -> Self {
        Self {
            select_move: |_| None,
            select_action: |_| None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitTag {
    Lurker,
}
