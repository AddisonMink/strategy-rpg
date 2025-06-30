use std::collections::HashSet;

use super::*;
use crate::util::*;

pub struct PlayerVision {
    tiles_visible: Vec<bool>,
    units_visible: HashSet<UnitId>,
}

impl PlayerVision {
    pub fn empty() -> Self {
        PlayerVision {
            tiles_visible: vec![false; (Map::WIDTH * Map::HEIGHT) as usize],
            units_visible: HashSet::new(),
        }
    }

    pub fn new(world: &World) -> Self {
        let mut tiles_visible = vec![false; (Map::WIDTH * Map::HEIGHT) as usize];
        let mut units_visible = HashSet::new();

        for player in world.player_units_iter() {
            for y in 0..Map::HEIGHT {
                for x in 0..Map::WIDTH {
                    let coord = Coord::new(x, y);
                    if world.unit_can_see_tile(player.id(), coord) {
                        tiles_visible[(coord.y * Map::WIDTH + coord.x) as usize] = true;
                    }
                }
            }

            for unit in world.units_iter() {
                if unit.data().side == Side::Player
                    || world.unit_can_see_unit(player.id(), unit.id())
                {
                    units_visible.insert(unit.id());
                }
            }
        }

        Self {
            tiles_visible,
            units_visible,
        }
    }

    pub fn tile_visible(&self, coord: Coord) -> bool {
        if Map::in_bounds(coord) {
            self.tiles_visible[(coord.y * Map::WIDTH + coord.x) as usize]
        } else {
            false
        }
    }

    pub fn unit_visible(&self, entity: UnitId) -> bool {
        self.units_visible.contains(&entity)
    }
}
