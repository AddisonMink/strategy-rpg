mod constants;
mod engine;
mod engine_v2;
mod game;
mod level_content;
mod level_model;
mod level_render;
mod level_system;
mod level_v2;
mod util;

use engine::*;
use game::*;
use macroquad::prelude::*;
use macroquad::rand::srand;

#[macroquad::main("Strategy RPG")]
async fn main() {
    engine_v2::asset::load_assets().await;

    let now = (macroquad::miniquad::date::now() * 1000.0) as u64;
    srand(now);

    let mut level = level_v2::Level::new();

    loop {
        let delta_time = get_frame_time();
        level.update(delta_time);
        clear_background(BLACK);
        level.draw();
        next_frame().await;
    }
}
