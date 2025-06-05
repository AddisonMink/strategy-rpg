mod battle;
mod data;
mod engine;
mod model;
mod prelude;
mod render;
mod update;

use battle::draw::*;
use battle::model::*;
use battle::update::*;
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
        movement: 3,
        coord,
    });

    battle.add_unit(Coord::new(4, 1), |id, coord| Unit {
        id,
        name: ShortString::new("Mr. A"),
        glyph: Glyph::new('A', RED),
        movement: 3,
        coord,
    });

    loop {
        let delta_time = get_frame_time();
        update_battle(&mut battle, delta_time);
        clear_background(BLACK);
        draw_battle(&battle);
        next_frame().await;
    }
}
