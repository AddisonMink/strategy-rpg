use crate::asset::MAP_FONT;
use crate::coord::Coord;
use crate::glyph::Glyph;
use macroquad::prelude::*;

const TILE_SIZE: f32 = 32.0;

pub fn draw_square(coord: Coord, color: Color) {
    let x = coord.x as f32 * TILE_SIZE;
    let y = coord.y as f32 * TILE_SIZE;
    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
}

pub fn draw_glyph(coord: Coord, glyph: Glyph) {
    let x = coord.x as f32 * TILE_SIZE;
    let y = coord.y as f32 * TILE_SIZE;
    let str = glyph.symbol.to_string();

    draw_text_ex(
        &str,
        x + TILE_SIZE / 8.0,
        y + TILE_SIZE - TILE_SIZE / 8.0,
        TextParams {
            font: MAP_FONT.get(),
            font_size: TILE_SIZE as u16,
            color: glyph.color,
            ..Default::default()
        },
    );
}
