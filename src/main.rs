use coord::Coord;
use entity::*;
use glyph::Glyph;
use light_grid::LightGrid;
use macroquad::prelude::*;
use map::Map;

mod algorithm;
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

    let unit_id = entities.next_id();
    entities.add_light(unit_id, 3);
    entities.add_position(unit_id, Coord { x: 14, y: 1 });

    entities.add_unit(
        unit_id,
        Glyph {
            symbol: '@',
            color: WHITE,
        },
    );

    let light_grid = LightGrid::new(&map, &entities);

    loop {
        clear_background(BLACK);

        for x in 0..Map::WIDTH {
            for y in 0..Map::HEIGHT {
                let coord = Coord { x, y };
                if light_grid.distance_from_light(coord) == 0 {
                    let tile = map.tile(coord);
                    if let Some(bg_color) = tile.background {
                        draw_grid::draw_square(coord, bg_color);
                    }
                    if let Some(unit) = entities.unit_at(coord) {
                        draw_grid::draw_glyph(coord, unit.glyph);
                    } else {
                        draw_grid::draw_glyph(coord, tile.glyph);
                    }
                }
            }
        }

        next_frame().await;
    }
}
