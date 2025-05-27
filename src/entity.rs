use crate::coord::Coord;
use std::collections::HashMap;

pub type EntityID = u16;

pub struct Light {
    pub id: EntityID,
    pub radius: u16,
}

pub struct Position {
    pub coord: Coord,
}

pub struct Entities {
    lights: HashMap<EntityID, Light>,
    positions: HashMap<EntityID, Position>,
    next_id: EntityID,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            lights: HashMap::new(),
            positions: HashMap::new(),
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

    pub fn get_position(&self, id: EntityID) -> Option<&Position> {
        self.positions.get(&id)
    }
}
