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

    let player_data = UnitData {
        name: ShortString::new("Hero"),
        glyph: Glyph::new('@', WHITE),
        side: Side::Player,
        movement: 3,
        hp_max: 5,
    };

    let mut battle = Battle::new(Map::new());

    battle.add_unit(Coord::new(1, 1), |id, coord| {
        Unit::new(id, coord, player_data)
    });

    battle.add_unit(Coord::new(4, 1), battle::content::npc::make_goon);

    loop {
        let delta_time = get_frame_time();
        update_battle(&mut battle, delta_time);
        clear_background(BLACK);
        draw_battle(&battle);
        next_frame().await;
    }
}
