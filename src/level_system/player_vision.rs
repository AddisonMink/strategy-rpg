use crate::engine::*;
use crate::level_model::*;
use std::collections::HashSet;

pub fn update_player_vision(level: &mut Level) {
    let mut tiles_visible = vec![false; (Map::WIDTH * Map::HEIGHT) as usize];

    let players = level
        .units
        .values()
        .filter(|unit| unit.side == Side::Player);

    for unit in players {
        for y in 0..Map::HEIGHT {
            for x in 0..Map::WIDTH {
                let coord = Coord::new(x, y);
                if level.unit_can_see_tile(unit.entity, coord) {
                    tiles_visible[(coord.y * Map::WIDTH + coord.x) as usize] = true;
                }
            }
        }
    }

    level.player_vision = PlayerVision {
        tiles_visible,
        units_visible: HashSet::new(),
    }
}
