mod engine;
mod game;
mod level_content;
mod level_model;
mod level_render;
mod level_system;

use engine::*;
use game::*;
use macroquad::prelude::*;
use macroquad::rand::srand;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let now = (macroquad::miniquad::date::now() * 1000.0) as u64;
    srand(now);

    let mut game = Game::new();

    loop {
        let delta_time = get_frame_time();
        update_game(&mut game, delta_time);
        clear_background(BLACK);
        render_game(&game);
        next_frame().await;
    }
}
