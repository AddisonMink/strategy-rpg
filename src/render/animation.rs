use crate::{prelude::*, util::asset::UI_FONT};

use super::draw_grid::TILE_SIZE;

const NUMBER_SIZE: u16 = 48;

pub fn draw_animation(game: &Game, animation: &Animation) {
    let progress = animation.elapsed / animation.duration;

    match animation.kind {
        AnimationKind::Number {
            coord,
            value,
            color,
        } => draw_number(coord, value, color, progress),
    }
}

fn draw_number(coord: Coord, value: i32, color: Color, progress: f32) {
    let text = value.to_string();
    let size = measure_text(&text, None, NUMBER_SIZE, 1.0);
    let alpha = 1.0 - progress.clamp(0.0, 1.0);
    let y_offset = NUMBER_SIZE as f32 * progress * -1.0;
    let x = coord.x as f32 * TILE_SIZE + (TILE_SIZE - size.width) / 2.0;
    let y = coord.y as f32 * TILE_SIZE + size.offset_y + NUMBER_SIZE as f32 + y_offset;
    draw_text_ex(
        &text,
        x,
        y,
        TextParams {
            font: UI_FONT.get(),
            font_size: NUMBER_SIZE,
            color: color.with_alpha(alpha),
            ..Default::default()
        },
    );
}
