use crate::asset::MAP_FONT;
use crate::tile::Tile;
use macroquad::prelude::*;

const TILE_SIZE: f32 = 32.0;

pub fn draw_tile(x: u16, y: u16, tile: &Tile) {
    let x = x as f32 * TILE_SIZE;
    let y = y as f32 * TILE_SIZE;
    let str = tile.glyph.symbol.to_string();

    if let Some(bg_color) = tile.background {
        draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, bg_color);
    }

    draw_text_ex(
        &str,
        x + TILE_SIZE / 8.0,
        y + TILE_SIZE - TILE_SIZE / 8.0,
        TextParams {
            font: MAP_FONT.get(),
            font_size: TILE_SIZE as u16,
            color: tile.glyph.color,
            ..Default::default()
        },
    );
}
