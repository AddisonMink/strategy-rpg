mod constants;
mod engine;
mod engine_v2;
mod game;
mod level_content;
mod level_model;
mod level_render;
mod level_system;
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

    let mut game = Game::new();

    loop {
        let delta_time = get_frame_time();
        //update_game(&mut game, delta_time);
        clear_background(BLACK);
        //render_game(&game);
        util::grid::draw_frame("Strategy RPG");
        next_frame().await;
    }
}
