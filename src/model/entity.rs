use crate::{coord::Coord, glyph::Glyph};
use std::collections::HashMap;

pub type EntityID = u16;

pub struct Light {
    pub id: EntityID,
    pub radius: u16,
}

pub struct Position {
    pub coord: Coord,
}

pub struct Unit {
    pub glyph: Glyph,
    pub vision: u16,
}

pub struct Entities {
    pub lights: HashMap<EntityID, Light>,
    pub positions: HashMap<EntityID, Position>,
    pub units: HashMap<EntityID, Unit>,
    next_id: EntityID,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            lights: HashMap::new(),
            positions: HashMap::new(),
            units: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn next_id(&mut self) -> EntityID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.positions
            .iter()
            .filter(|(_, pos)| pos.coord == coord)
            .find_map(|(&id, _)| self.units.get(&id))
    }
}
