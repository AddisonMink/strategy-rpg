use super::*;
use crate::util::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    NPC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitTag {
    Lurker,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitData {
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    pub vision: u16,
    pub movement: u16,
    pub hp_max: u16,
    pub tags: ShortList<UnitTag>,
    pub behavior: Option<UnitBehavior>,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitBehavior {
    pub select_move: fn(&World) -> Option<VecDeque<Coord>>,
    pub select_action: fn(&World) -> Option<VecDeque<Effect>>,
}

impl Default for UnitBehavior {
    fn default() -> Self {
        Self {
            select_move: |_| None,
            select_action: |_| None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub visible_players: HashSet<UnitId>,
    pub last_seen_player: Option<(UnitId, Coord)>,
}

#[derive(Debug, Clone)]
pub struct Unit {
    id: UnitId,
    data: UnitData,
    behavior: UnitBehavior,
    pub coord: Coord,
    pub hp: u16,
    pub memory: Memory,
    pub items: HashMap<ItemId, Item>,
    pub light: Option<Light>,
    pub tags: HashSet<UnitTag>,
}

impl Unit {
    pub fn new(id: UnitId, data: UnitData, coord: Coord) -> Self {
        let tag_set = data
            .tags
            .iter()
            .map(|&tag| tag)
            .collect::<HashSet<UnitTag>>();

        Self {
            id,
            data,
            coord,
            hp: data.hp_max,
            behavior: data.behavior.unwrap_or_default(),
            memory: Memory::default(),
            items: HashMap::default(),
            light: None,
            tags: tag_set,
        }
    }

    pub fn id(&self) -> UnitId {
        self.id
    }

    pub fn data(&self) -> &UnitData {
        &self.data
    }

    pub fn behavior(&self) -> &UnitBehavior {
        &self.behavior
    }

    pub fn actions(&self) -> Vec<ItemAction> {
        self.items
            .values()
            .flat_map(|item| item.actions())
            .collect()
    }
}
