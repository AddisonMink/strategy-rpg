use super::Effect;
use super::Level;
use crate::engine::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub entity: Entity,
    pub coord: Coord,
}

impl Position {
    pub fn new(entity: Entity, coord: Coord) -> Self {
        Self { entity, coord }
    }
}

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
    ) -> Self {
        Self {
            entity,
            name,
            glyph,
            side,
            vision,
            movement,
            hp_max,
            hp: hp_max, // Start with full HP
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    NPC,
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub entity: Entity,
    pub radius: u16,
    pub color: Color,
}

impl Light {
    pub fn new(entity: Entity, radius: u16, color: Color) -> Self {
        Self {
            entity,
            radius,
            color,
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
