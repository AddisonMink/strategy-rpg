use super::behavior::*;
use crate::model::*;
use macroquad::prelude::*;
use std::collections::VecDeque;

pub fn make_goon(id: UnitId, coord: Coord) -> Unit {
    let select_move = |unit: &Unit, game: &Game| -> Option<VecDeque<Coord>> {
        let player = find_nearest_visible_player(game, unit.coord, unit.vision)?;
        let mut path = find_path_to_adjacent(game, unit.coord, player.coord);
        path.truncate(unit.movement as usize);
        Some(path)
    };

    Unit {
        id,
        is_player: false,
        glyph: Glyph::new('G', GREEN),
        name: "Goon".to_string(),
        vision: 1,
        movement: 2,
        coord,
        light: None,
        npc_behavior: Some(NpcBehavior { select_move }),
    }
}

pub fn make_player(id: UnitId, coord: Coord) -> Unit {
    Unit {
        id,
        is_player: true,
        glyph: Glyph::new('@', WHITE),
        name: "Player".to_string(),
        vision: 2,
        movement: 3,
        coord,
        light: None,
        npc_behavior: None,
    }
}
