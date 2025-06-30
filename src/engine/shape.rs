use super::color::*;

pub fn draw_rectangle(x: f32, y: f32, width: f32, height: f32, color: Color) {
    macroquad::prelude::draw_rectangle(x, y, width, height, color);
}
