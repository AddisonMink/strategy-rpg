use super::behavior::*;
use crate::prelude::*;
use std::collections::VecDeque;

pub fn make_goon(id: UnitId, coord: Coord) -> Unit {
    let glyph = Glyph::new('g', GREEN);
    let vision = 1;
    let movement = 2;
    let hp_max = 5;

    let select_move = |unit: &Unit, game: &Game| -> Option<VecDeque<Coord>> {
        let player = find_nearest_visible_player(game, unit.coord, unit.vision)?;
        let mut path = find_path_to_adjacent(game, unit.coord, player.coord);
        path.truncate(unit.movement as usize);
        Some(path)
    };

    Unit {
        id,
        is_player: false,
        glyph,
        name: "Goon".to_string(),
        vision,
        movement,
        hp_max,
        coord,
        hp: hp_max,
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
        hp_max: 10,
        coord,
        hp: 10,
        light: None,
        npc_behavior: None,
    }
}
