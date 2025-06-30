mod constants;
mod engine_v2;
mod level_v2;
mod util;

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
        let result = level.update(delta_time);

        if let Some(level_v2::LevelResult::Restart) = result {
            level = level_v2::Level::new();
        }

        clear_background(BLACK);
        level.draw();
        next_frame().await;
    }
}
