use macroquad::prelude::*;

use crate::engine::{Coord, Glyph, Panel, asset};

pub const WIDTH: u16 = 10;
pub const HEIGHT: u16 = 10;
pub const TILE_SIZE: f32 = 32.0;

const GRID_FRAME_ORIGIN: Vec2 = Vec2::new(10.0, 10.0);
const GRID_ORIGIN: Vec2 = Vec2::new(20.0, 20.0);
const TEXT_SIZE: u16 = 32;
const BIG_TEXT_SIZE: u16 = 48;
const WIDTH_PX: f32 = WIDTH as f32 * TILE_SIZE + 40.0;
const HEIGHT_PX: f32 = HEIGHT as f32 * TILE_SIZE + 40.0;

pub fn mouse_coord() -> Option<Coord> {
    let (x, y) = mouse_position();
    let x = ((x - GRID_ORIGIN.x) / TILE_SIZE).floor() as u16;
    let y = ((y - GRID_ORIGIN.y) / TILE_SIZE).floor() as u16;
    let c = Coord::new(x, y);
    in_bounds(c).then_some(c)
}

pub fn draw_frame(title: &str) {
    let width = WIDTH as f32 * TILE_SIZE + 20.0;
    let height = HEIGHT as f32 * TILE_SIZE + 20.0;
    let panel = Panel::empty(title, WHITE, width, height);

    panel.draw(GRID_FRAME_ORIGIN.x, GRID_FRAME_ORIGIN.y);
}

pub fn draw_square(coord: Coord, color: Color) {
    let (x, y) = coord_to_pos(coord);
    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
}

pub fn draw_glyph(coord: Coord, glyph: Glyph) {
    draw_glyph_with_offset(coord, glyph, (0.0, 0.0));
}

pub fn draw_glyph_with_offset(coord: Coord, glyph: Glyph, offset: (f32, f32)) {
    let str = glyph.symbol.to_string();
    let (cx, cy) = coord_to_pos(coord);
    let x = cx + TILE_SIZE / 8.0 + offset.0;
    let y = cy + TILE_SIZE - TILE_SIZE / 8.0 + offset.1;

    draw_text_ex(
        &str,
        x,
        y,
        TextParams {
            font: asset::MAP_FONT.get(),
            font_size: TILE_SIZE as u16,
            color: glyph.color,
            ..Default::default()
        },
    );
}

pub fn draw_text_with_offset(coord: Coord, text: &str, color: Color, offset: (f32, f32)) {
    let (cx, cy) = coord_to_pos(coord);
    let size = measure_text(text, asset::UI_FONT.get(), TEXT_SIZE, 1.0);
    let x = cx + (TILE_SIZE - size.width) / 2.0 + offset.0;
    let y = cy + (TILE_SIZE - size.height) / 2.0 + size.offset_y + offset.1;

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: asset::UI_FONT.get(),
            font_size: TEXT_SIZE,
            color,
            ..Default::default()
        },
    );
}

pub fn draw_panel(panel: &Panel, coord: Coord) {
    let (x, y) = coord_to_pos(coord);
    let width = panel.get_width();
    let height = panel.get_height();
    let x = x + (TILE_SIZE - width) / 2.0;
    let y = y + (TILE_SIZE - height) / 2.0;
    panel.draw(x, y);
}

pub fn draw_big_message(message: String, color: Color) {
    let size = measure_text(&message, asset::UI_FONT.get(), BIG_TEXT_SIZE, 1.0);
    let x = (WIDTH_PX - size.width) / 2.0;
    let y = (HEIGHT_PX - size.height) / 2.0 + size.offset_y;

    draw_text_ex(
        &message,
        x,
        y,
        TextParams {
            font: asset::UI_FONT.get(),
            font_size: BIG_TEXT_SIZE,
            color,
            ..Default::default()
        },
    );
}

pub fn in_bounds(coord: Coord) -> bool {
    coord.x < WIDTH && coord.y < HEIGHT
}

pub fn coord_to_pos(coord: Coord) -> (f32, f32) {
    let x = GRID_ORIGIN.x + coord.x as f32 * TILE_SIZE;
    let y = GRID_ORIGIN.y + coord.y as f32 * TILE_SIZE;
    (x, y)
}
