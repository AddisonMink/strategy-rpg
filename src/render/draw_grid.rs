use super::panel::Panel;
use crate::asset::MAP_FONT;
use crate::coord::Coord;
use crate::glyph::Glyph;
use macroquad::prelude::*;

const ORIGIN: Vec2 = Vec2::new(20.0, 20.0);
const TILE_SIZE: f32 = 32.0;

pub fn draw_panel() {
    let panel = Panel::empty("MAP", WHITE, 532.0, 244.0);
    panel.draw(10.0, 10.0);
}

pub fn draw_square(coord: Coord, color: Color) {
    let x = coord.x as f32 * TILE_SIZE + ORIGIN.x;
    let y = coord.y as f32 * TILE_SIZE + ORIGIN.y;
    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
}

pub fn draw_string(coord: Coord, text: &str, color: Color) {
    let x = coord.x as f32 * TILE_SIZE;
    let y = coord.y as f32 * TILE_SIZE;

    draw_text_ex(
        text,
        x + TILE_SIZE / 8.0 + ORIGIN.x,
        y + TILE_SIZE - TILE_SIZE / 8.0 + ORIGIN.y,
        TextParams {
            font: MAP_FONT.get(),
            font_size: TILE_SIZE as u16,
            color,
            ..Default::default()
        },
    );
}

pub fn draw_glyph(coord: Coord, glyph: Glyph) {
    let str = glyph.symbol.to_string();
    draw_string(coord, &str, glyph.color);
}
