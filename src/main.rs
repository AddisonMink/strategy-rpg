use coord::Coord;
use entity::*;
use light_grid::LightGrid;
use macroquad::prelude::*;
use map::Map;

mod asset;
mod coord;
mod draw_grid;
mod entity;
mod glyph;
mod light_grid;
mod map;
mod tile;

#[macroquad::main("Strategy RPG")]
async fn main() {
    asset::load_assets().await;

    let map = map::Map::new();
    let mut entities = Entities::new();

    let light_id = entities.next_id();
    entities.add_light(light_id, 5);
    entities.add_position(light_id, Coord { x: 1, y: 1 });

    let light_grid = LightGrid::new(&entities);

    loop {
        clear_background(BLACK);

        for x in 0..Map::WIDTH {
            for y in 0..Map::HEIGHT {
                let coord = Coord { x, y };
                if light_grid.light_value(coord) == 0 {
                    let tile = map.tile(coord);
                    draw_grid::draw_tile(x, y, tile);
                }
            }
        }

        next_frame().await;
    }
}
