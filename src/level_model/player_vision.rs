use std::collections::HashSet;

use super::entity::Entity;
use super::map::Map;
use crate::engine::*;

pub struct PlayerVision {
    pub tiles_visible: Vec<bool>,
    pub units_visible: HashSet<Entity>,
}

impl PlayerVision {
    pub fn empty() -> Self {
        PlayerVision {
            tiles_visible: vec![false; (Map::WIDTH * Map::HEIGHT) as usize],
            units_visible: HashSet::new(),
        }
    }

    pub fn tile_visible(&self, coord: Coord) -> bool {
        if Map::in_bounds(coord) {
            self.tiles_visible[(coord.y * grid::WIDTH + coord.x) as usize]
        } else {
            false
        }
    }

    pub fn entity_visible(&self, entity: Entity) -> bool {
        self.units_visible.contains(&entity)
    }
}
