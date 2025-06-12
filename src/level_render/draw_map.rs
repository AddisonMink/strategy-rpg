use crate::engine::*;
use crate::level_model::*;

pub fn draw_map(level: &Level) {
    grid::draw_frame("MAP");
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
        .positions
        .values()
        .filter(|p| animating_entitiy.is_none() || p.entity != animating_entitiy.unwrap())
        .filter(|p| level.player_vision.tile_visible(p.coord))
        .filter_map(|p| level.units.get(&p.entity).map(|unit| (p, unit)))
        .for_each(|(pos, unit)| {
            let distance_from_light = level.light_grid.distance_from_light(pos.coord);
            let light_color = if distance_from_light == 0 {
                level.light_grid.light_color(pos.coord)
            } else {
                BLACK
            };

            let glyph_color = color::mix_color(unit.glyph.color, light_color, 0.5);
            let glyph = Glyph::new(unit.glyph.symbol, glyph_color);
            grid::draw_glyph(pos.coord, glyph);
        });
}
