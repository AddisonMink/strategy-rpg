mod constants;
mod engine;
mod level;
mod util;

use macroquad::prelude::*;
use macroquad::rand::srand;

#[macroquad::main("Strategy RPG")]
async fn main() {
    engine::asset::load_assets().await;

    let now = (macroquad::miniquad::date::now() * 1000.0) as u64;
    srand(now);

    let mut level = level::Level::new();

    loop {
        let delta_time = get_frame_time();
        let result = level.update(delta_time);

        if let Some(level::LevelResult::Restart) = result {
            level = level::Level::new();
        }

        clear_background(BLACK);
        level.draw();
        next_frame().await;
    }
}
