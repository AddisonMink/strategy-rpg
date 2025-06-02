use crate::{prelude::*, util::asset::UI_FONT};

use super::draw_grid::{ORIGIN, TILE_SIZE};

const NUMBER_SIZE: u16 = 32;

pub fn draw_animation(game: &Game, animation: &Animation) {
    let progress = animation.elapsed / animation.duration;

    match &animation.kind {
        AnimationKind::Number {
            coord,
            value,
            color,
        } => draw_number(*coord, *value, *color, progress),
        AnimationKind::Meter {
            coord,
            label,
            value,
            max_value,
            color,
        } => draw_meter(*coord, label, *value, *max_value, *color),
    }
}

fn draw_number(coord: Coord, value: i32, color: Color, progress: f32) {
    let text = value.to_string();
    let size = measure_text(&text, None, NUMBER_SIZE, 1.0);
    let alpha = 1.0 - progress.clamp(0.0, 1.0);
    let y_offset = NUMBER_SIZE as f32 * progress * -1.0;
    let x = ORIGIN.x + coord.x as f32 * TILE_SIZE - size.width / 2.0;
    let y = ORIGIN.y + coord.y as f32 * TILE_SIZE + size.offset_y + TILE_SIZE / 3.0 + y_offset;
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

pub fn draw_meter(coord: Coord, label: &str, value: u16, max_value: u16, color: Color) {
    let panel = Panel::builder("", WHITE)
        .short_meter(label, WHITE, value, max_value, color)
        .build();

    let x = ORIGIN.x + coord.x as f32 * TILE_SIZE + TILE_SIZE / 2.0 - panel.get_width() / 2.0;
    let y = ORIGIN.y + coord.y as f32 * TILE_SIZE + TILE_SIZE / 2.0 - panel.get_height() / 2.0;

    panel.draw(x, y);
}
