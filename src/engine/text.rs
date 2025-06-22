use super::asset;
use crate::constants::{TEXT_SIZE, TILE_SIZE};
use macroquad::color::Color;
use macroquad::text::{draw_text_ex, measure_text};

const MAP_FONT_SIZE: u16 = TILE_SIZE as u16;

pub fn text_size(text: &str) -> (f32, f32) {
    let size = measure_text(text, asset::UI_FONT.get(), TEXT_SIZE, 1.0);
    (size.width, size.height)
}

pub fn draw_text(x: f32, y: f32, text: &str, color: Color) {
    let size = measure_text(text, asset::UI_FONT.get(), TEXT_SIZE, 1.0);

    draw_text_ex(
        text,
        x,
        y + size.offset_y,
        macroquad::text::TextParams {
            font: asset::UI_FONT.get(),
            font_size: TEXT_SIZE,
            color,
            ..Default::default()
        },
    );
}

pub fn draw_glyph(x: f32, y: f32, symbol: char, color: Color) {
    let text = symbol.to_string();
    let size = measure_text(&text, asset::MAP_FONT.get(), MAP_FONT_SIZE, 1.0);

    draw_text_ex(
        &text,
        x,
        y + size.offset_y,
        macroquad::text::TextParams {
            font: asset::MAP_FONT.get(),
            font_size: MAP_FONT_SIZE,
            color,
            ..Default::default()
        },
    );
}
