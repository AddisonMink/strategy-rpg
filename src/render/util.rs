use macroquad::prelude::*;

pub fn mix_color(primary: Color, secondary: Color, ratio: f32) -> Color {
    let r = primary.r * (1.0 - ratio) + secondary.r * ratio;
    let g = primary.g * (1.0 - ratio) + secondary.g * ratio;
    let b = primary.b * (1.0 - ratio) + secondary.b * ratio;
    let a = primary.a * (1.0 - ratio) + secondary.a * ratio;
    Color { r, g, b, a }
}
