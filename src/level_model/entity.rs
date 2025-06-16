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

#[derive(Debug, Clone, Copy)]
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
    ) -> Self {
        Self {
            entity,
            name,
            glyph,
            side,
            vision,
            movement,
            hp_max,
            hp: hp_max, // Start with full HP,
            coord,
            light: None, // No light by default
        }
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

#[derive(Debug, Clone)]
pub struct VisionMemory {
    pub entity: Entity,
    pub visible_players: HashSet<Entity>,
    pub last_seen_player: Option<(Entity, Coord)>,
}

impl VisionMemory {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            visible_players: HashSet::new(),
            last_seen_player: None,
        }
    }
}

pub struct Behavior {
    pub entity: Entity,
    pub select_move: fn(&Level) -> Option<VecDeque<Coord>>,
    pub select_action: fn(&Level) -> Option<VecDeque<Effect>>,
}

pub struct Inventory {
    pub entity: Entity,
    pub items: HashMap<ItemId, Item>,
}

impl Inventory {
    pub fn new<I>(entity: Entity, items: I) -> Self
    where
        I: IntoIterator<Item = Item>,
    {
        let items = items.into_iter().map(|item| (item.id, item)).collect();
        Self { entity, items }
    }
}
