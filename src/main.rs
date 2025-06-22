mod constants;
mod engine;
mod game;
mod level_content;
mod level_model;
mod level_render;
mod level_system;
mod ui;
mod engine_v2;

use engine::*;
use game::*;
use macroquad::prelude::*;
use macroquad::rand::srand;
use ui::*;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let now = (macroquad::miniquad::date::now() * 1000.0) as u64;
    srand(now);

    let mut game = Game::new();

    loop {
        let delta_time = get_frame_time();
        //update_game(&mut game, delta_time);
        clear_background(BLACK);
        //render_game(&game);
        ui::Panel::builder()
            .title("Hello", WHITE)
            .text("Line 1", WHITE)
            .selectable_text("Line 2", WHITE)
            .labeled_meter("HP", 3, 2, 5, RED)
            .position(10.0, 10.0)
            .build()
            .draw();
        next_frame().await;
    }
}
