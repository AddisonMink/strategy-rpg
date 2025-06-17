use crate::engine::*;
use crate::level_model::*;

pub fn draw_map(level: &Level, title: &str) {
    grid::draw_frame(title);
    for y in 0..Map::HEIGHT {
        for x in 0..Map::WIDTH {
            let coord = Coord::new(x, y);
            if !level.player_vision.tile_visible(coord) {
                continue;
            }

            let distance_from_light = level.light_grid.distance_from_light(coord);
            let tile = level.map.tile(coord);
            let unoccupied = level.unit_at(coord).is_none();

            let light_color = if distance_from_light == 0 {
                level.light_grid.light_color(coord)
            } else {
                BLACK
            };

            if let Some(color) = tile.bg_color {
                grid::draw_square(coord, color::mix_color(color, light_color, 0.5));
            }

            if unoccupied {
                let glyph_color = color::mix_color(tile.glyph.color, light_color, 0.5);
                let glyph = Glyph::new(tile.glyph.symbol, glyph_color);
                grid::draw_glyph(coord, glyph);
            }
        }
    }

    let animating_entitiy = level
        .animation_queue
        .front()
        .and_then(|a| a.animating_entity());

    level
        .units
        .values()
        .filter(|u| animating_entitiy.is_none() || u.entity != animating_entitiy.unwrap())
        .filter(|u| level.player_vision.entity_visible(u.entity))
        .for_each(|unit| {
            let distance_from_light = level.light_grid.distance_from_light(unit.coord);
            let light_color = if distance_from_light == 0 {
                level.light_grid.light_color(unit.coord)
            } else {
                BLACK
            };

            let glyph_color = color::mix_color(unit.glyph.color, light_color, 0.5);
            let glyph = Glyph::new(unit.glyph.symbol, glyph_color);
            grid::draw_glyph(unit.coord, glyph);
        });
}
