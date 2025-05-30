use super::Panel;
use super::draw_grid;
use super::util::*;
use crate::model::*;
use macroquad::prelude::*;

pub fn draw_map(game: &Game, flicker: f32) {
    for x in 0..Map::WIDTH {
        for y in 0..Map::HEIGHT {
            let coord = Coord { x, y };
            let distance_from_light = game.light_grid.distance_from_light(coord);
            let light_color = game.light_grid.color_at(coord).with_alpha(flicker);

            if game.player_can_see(coord) {
                let tile = game.map.tile(coord);

                if let Some(bg_color) = tile.background {
                    draw_grid::draw_square(coord, mix_color(bg_color, light_color, 0.5));
                }

                if let Some(unit) = game.unit_at(coord) {
                    let glyph = Glyph {
                        symbol: unit.glyph.symbol,
                        color: mix_color(unit.glyph.color, light_color, 0.5),
                    };
                    draw_grid::draw_glyph(coord, glyph);
                } else {
                    let glyph = Glyph {
                        symbol: tile.glyph.symbol,
                        color: mix_color(tile.glyph.color, light_color, 0.5),
                    };
                    draw_grid::draw_glyph(coord, glyph);
                }

                if distance_from_light > 0 {
                    draw_grid::draw_square(coord, BLACK.with_alpha(0.5));
                }
            }
        }
    }
}
