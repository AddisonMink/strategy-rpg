use super::Effect;
use super::Level;
use super::item::*;
use crate::engine::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub radius: u16,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u16);

#[derive(Debug, Clone)]
pub struct Tags {
    pub entity: Entity,
    pub tags: HashSet<EntityTag>,
}

impl Tags {
    pub fn new(entity: Entity, tags: &ShortList<EntityTag>) -> Self {
        Self {
            entity,
            tags: tags.as_slice().iter().cloned().collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityTag {
    Lurker,
}

#[derive(Debug, Clone)]
pub struct Unit {
    // Identifiers
    pub entity: Entity,
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    // Attributes
    pub vision: u16,
    pub movement: u16,
    pub hp_max: u16,
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
        entity: Entity,
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

#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub entity: Entity,
    pub coord: Coord,
    pub light: Light,
}

impl PointLight {
    pub fn new(entity: Entity, coord: Coord, light: Light) -> Self {
        Self {
            entity,
            coord,
            light,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub visible_players: HashSet<Entity>,
    pub last_seen_player: Option<(Entity, Coord)>,
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
