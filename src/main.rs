use macroquad::prelude::*;
use map::Map;

mod asset;
mod draw_grid;
mod glyph;
mod map;
mod tile;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let map = map::Map::new();

    loop {
        clear_background(BLACK);

        for x in 0..Map::WIDTH {
            for y in 0..Map::HEIGHT {
                let tile = map.get_tile(x, y);
                draw_grid::draw_tile(x, y, tile);
            }
        }

        next_frame().await;
    }
}
