mod engine;
mod level_content;
mod level_model;
mod level_render;
mod level_system;

use engine::*;
use level_content::*;
use level_model::*;
use level_render::*;
use level_system::*;
use macroquad::prelude::*;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let mut level = Level::empty();
    add_hero(&mut level, Coord::new(1, 1));
    add_goon(&mut level, Coord::new(5, 1));
    add_shadow(&mut level, Coord::new(8, 4));
    add_point_light(&mut level, Coord::new(1, 1), 3, BLUE);

    loop {
        let delta_time = get_frame_time();
        while update_level(&mut level, delta_time) == UpdateResult::Continue {}
        clear_background(BLACK);
        render_level(&level);
        next_frame().await;
    }
}
