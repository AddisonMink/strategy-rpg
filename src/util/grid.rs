use super::*;
use crate::constants::*;
use crate::engine_v2::*;

pub fn coords_iter() -> impl Iterator<Item = Coord> {
    (0..GRID_ROWS).flat_map(move |y| (0..GRID_COLUMNS).map(move |x| Coord::new(x, y)))
}

pub fn mouse_coord() -> Option<Coord> {
    let (x, y) = mouse_pos();
    let x = ((x - GRID_ORIGIN.0) / TILE_SIZE).floor() as u16;
    let y = ((y - GRID_ORIGIN.1) / TILE_SIZE).floor() as u16;
    let c = Coord::new(x, y);
    in_bounds(c).then_some(c)
}

pub fn draw_frame(title: &str) {
    Panel::builder()
        .title(title, WHITE)
        .size(GRID_PANE_WIDTH, GRID_PANE_HEIGHT)
        .position(GRID_PANE_ORIGIN.0, GRID_PANE_ORIGIN.1)
        .build()
        .draw();
}

pub fn draw_square(coord: Coord, color: Color) {
    let (x, y) = coord_to_pos(coord);
    draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, color);
}

pub fn draw_glyph(coord: Coord, glyph: Glyph) {
    draw_glyph_with_offset(coord, glyph, (0.0, 0.0));
}

pub fn draw_glyph_with_offset(coord: Coord, glyph: Glyph, offset: (f32, f32)) {
    let (cx, cy) = coord_to_pos(coord);
    let x = cx + offset.0 + TILE_SIZE / 8.0;
    let y = cy + offset.1 + TILE_SIZE / 8.0;
    crate::engine_v2::draw_glyph(x, y, glyph.symbol, glyph.color);
}

pub fn draw_text(coord: Coord, text: &str, color: Color) {
    draw_text_with_offset(coord, text, color, (0.0, 0.0));
}

pub fn draw_text_with_offset(coord: Coord, text: &str, color: Color, offset: (f32, f32)) {
    let (cx, cy) = coord_to_pos(coord);
    let (text_width, text_height) = big_text_size(text);
    let x = cx + (TILE_SIZE - text_width) / 2.0 + offset.0;
    let y = cy + (TILE_SIZE - text_height) / 2.0 + offset.1;
    crate::engine_v2::draw_big_text(x, y, text, color);
}

pub fn draw_text_centered(text: &str, sub_text: Option<&str>, color: Color) {
    let (text_width, text_height) = huge_text_size(text);
    let x = (GRID_PANE_WIDTH - text_width) / 2.0 + GRID_PANE_ORIGIN.0;
    let y = (GRID_PANE_HEIGHT - text_height) / 2.0 + GRID_PANE_ORIGIN.1;

    crate::engine_v2::draw_huge_text(x, y, text, color);

    if let Some(sub_text) = sub_text {
        let (sub_text_width, _) = text_size(sub_text);
        let sub_x = (GRID_PANE_WIDTH - sub_text_width) / 2.0 + GRID_PANE_ORIGIN.0;
        let sub_y = y + text_height + PADDING;
        crate::engine_v2::draw_text(sub_x, sub_y, sub_text, color);
    }
}

pub fn in_bounds(coord: Coord) -> bool {
    coord.x < GRID_COLUMNS && coord.y < GRID_ROWS
}

pub fn coord_to_pos(coord: Coord) -> (f32, f32) {
    let x = GRID_ORIGIN.0 + coord.x as f32 * TILE_SIZE;
    let y = GRID_ORIGIN.1 + coord.y as f32 * TILE_SIZE;
    (x, y)
}
