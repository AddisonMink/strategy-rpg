use macroquad::prelude::*;

pub fn add_colors(primary: Color, secondary: Color) -> Color {
    let r = primary.r * primary.a + secondary.r * secondary.a;
    let g = primary.g * primary.a + secondary.g * secondary.a;
    let b = primary.b * primary.a + secondary.b * secondary.a;
    Color { r, g, b, a: 1.0 }
}

pub fn normalize_color(color: Color) -> Color {
    let max_value = color.r.max(color.g).max(color.b);
    if max_value > 1.0 {
        Color {
            r: color.r / max_value,
            g: color.g / max_value,
            b: color.b / max_value,
            a: color.a,
        }
    } else {
        color
    }
}

pub fn mix_color(primary: Color, secondary: Color, ratio: f32) -> Color {
    let r = primary.r * (1.0 - ratio) + secondary.r * ratio;
    let g = primary.g * (1.0 - ratio) + secondary.g * ratio;
    let b = primary.b * (1.0 - ratio) + secondary.b * ratio;
    let a = primary.a * (1.0 - ratio) + secondary.a * ratio;
    Color { r, g, b, a }
}
