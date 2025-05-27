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
    lights: HashMap<EntityID, Light>,
    positions: HashMap<EntityID, Position>,
    units: HashMap<EntityID, Unit>,
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

    pub fn add_light(&mut self, id: EntityID, radius: u16) {
        let light = Light { id, radius };
        self.lights.insert(id, light);
    }

    pub fn iter_lights(&self) -> impl Iterator<Item = &Light> {
        self.lights.values()
    }

    pub fn add_position(&mut self, id: EntityID, coord: Coord) {
        let position = Position { coord };
        self.positions.insert(id, position);
    }

    pub fn position(&self, id: EntityID) -> Option<&Position> {
        self.positions.get(&id)
    }

    pub fn position_mut(&mut self, id: EntityID) -> Option<&mut Position> {
        self.positions.get_mut(&id)
    }

    pub fn add_unit(&mut self, id: EntityID, glyph: Glyph, vision: u16) {
        let unit = Unit { glyph, vision };
        self.units.insert(id, unit);
    }

    pub fn unit(&self, id: EntityID) -> Option<&Unit> {
        self.units.get(&id)
    }

    pub fn unit_at(&self, coord: Coord) -> Option<&Unit> {
        self.positions.iter().find_map(|(&id, pos)| {
            if pos.coord == coord {
                self.units.get(&id)
            } else {
                None
            }
        })
    }
}
