use crate::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

const ORIGIN: Vec2 = Vec2::new(20.0, 20.0);
const FRAME_WIDTH: f32 = 532.0;
const FRAME_HEIGHT: f32 = 244.0;

pub fn draw_frame_panel() {
    let panel = Panel::empty("MAP", WHITE, FRAME_WIDTH, FRAME_HEIGHT);
    panel.draw(10.0, 10.0);
}

pub fn draw_panel_centered(panel: &Panel) {
    let x = ORIGIN.x + (FRAME_WIDTH - panel.get_width()) / 2.0;
    let y = ORIGIN.y + (FRAME_HEIGHT - panel.get_height()) / 2.0;
    panel.draw(x, y);
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
            font: asset::MAP_FONT.get(),
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
