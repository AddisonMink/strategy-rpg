mod battle;
mod data;
mod engine;
mod model;
mod prelude;
mod render;
mod update;

use battle::draw::*;
use battle::model::*;
use engine::*;
use macroquad::prelude::*;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let mut battle = Battle::new(Map::new());

    battle.add_unit(Coord::new(1, 1), |id, coord| Unit {
        id,
        name: ShortString::new("Hero"),
        glyph: Glyph::new('@', WHITE),
        coord,
    });

    battle.add_unit(Coord::new(8, 8), |id, coord| Unit {
        id,
        name: ShortString::new("Mr. A"),
        glyph: Glyph::new('A', RED),
        coord,
    });

    loop {
        clear_background(BLACK);
        draw_battle(&battle);
        next_frame().await;
    }
}
