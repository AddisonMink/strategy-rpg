use super::asset;
use crate::constants::*;
use macroquad::color::Color;
use macroquad::text::{draw_text_ex, measure_text};

pub fn text_size(text: &str) -> (f32, f32) {
    let size = measure_text(text, asset::UI_FONT.get(), TEXT_SIZE, 1.0);
    (size.width, size.height)
}

pub fn big_text_size(text: &str) -> (f32, f32) {
    let size = measure_text(text, asset::MAP_FONT.get(), BIG_TEXT_SIZE, 1.0);
    (size.width, size.height)
}

pub fn huge_text_size(text: &str) -> (f32, f32) {
    let size = measure_text(text, asset::UI_FONT.get(), HUGE_TEXT_SIZE, 1.0);
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

pub fn draw_big_text(x: f32, y: f32, text: &str, color: Color) {
    let size = measure_text(text, asset::MAP_FONT.get(), BIG_TEXT_SIZE, 1.0);

    draw_text_ex(
        text,
        x,
        y + size.offset_y,
        macroquad::text::TextParams {
            font: asset::MAP_FONT.get(),
            font_size: BIG_TEXT_SIZE,
            color,
            ..Default::default()
        },
    );
}

pub fn draw_huge_text(x: f32, y: f32, text: &str, color: Color) {
    let size = measure_text(text, asset::UI_FONT.get(), HUGE_TEXT_SIZE, 1.0);

    draw_text_ex(
        text,
        x,
        y + size.offset_y,
        macroquad::text::TextParams {
            font: asset::UI_FONT.get(),
            font_size: HUGE_TEXT_SIZE,
            color,
            ..Default::default()
        },
    );
}

pub fn draw_glyph(x: f32, y: f32, symbol: char, color: Color) {
    let text = symbol.to_string();
    let size = measure_text(&text, asset::MAP_FONT.get(), BIG_TEXT_SIZE, 1.0);

    draw_text_ex(
        &text,
        x,
        y + size.offset_y,
        macroquad::text::TextParams {
            font: asset::MAP_FONT.get(),
            font_size: BIG_TEXT_SIZE,
            color,
            ..Default::default()
        },
    );
}
