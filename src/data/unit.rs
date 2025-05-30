use crate::model::*;
use macroquad::prelude::*;

pub fn make_goon(id: UnitId, coord: Coord) -> Unit {
    Unit {
        id,
        is_player: false,
        glyph: Glyph::new('G', GREEN),
        name: "Goon".to_string(),
        vision: 1,
        movement: 2,
        coord,
        light: None,
        npc_behavior: None,
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
