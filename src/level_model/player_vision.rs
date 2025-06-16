use std::collections::HashSet;

use super::map::Map;
use super::unit::UnitId;
use crate::engine::*;

pub struct PlayerVision {
    pub tiles_visible: Vec<bool>,
    pub units_visible: HashSet<UnitId>,
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

    pub fn entity_visible(&self, entity: UnitId) -> bool {
        self.units_visible.contains(&entity)
    }
}
