use crate::{prelude::*, util::asset::UI_FONT};

use super::draw_grid::{ORIGIN, TILE_SIZE};

const NUMBER_SIZE: u16 = 32;
const MESSAGE_SIZE: u16 = 24;

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
        AnimationKind::Message { coord, text, color } => {
            draw_message(*coord, text, *color, progress)
        }
        AnimationKind::PanelMessage { coord, title, text } => {
            draw_panel_message(*coord, title, text);
        }
    }
}

fn draw_number(coord: Coord, value: i32, color: Color, progress: f32) {
    let text = value.to_string();
    let alpha = 1.0 - progress.clamp(0.0, 1.0);
    let y_offset = NUMBER_SIZE as f32 * progress * -1.0;
    let (x, y) = text_pos(coord, &text, NUMBER_SIZE, y_offset);

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

    let (x, y) = panel_pos(coord, &panel);

    panel.draw(x, y);
}

pub fn draw_message(coord: Coord, text: &str, color: Color, progress: f32) {
    let (x, y) = text_pos(coord, text, MESSAGE_SIZE, 0.0);
    let alpha = 1.0 - progress.clamp(0.0, 1.0);

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: UI_FONT.get(),
            font_size: MESSAGE_SIZE,
            color: color.with_alpha(alpha),
            ..Default::default()
        },
    );
}

pub fn draw_panel_message(coord: Coord, title: &str, text: &str) {
    let panel = Panel::builder(title, WHITE).line(text, WHITE).build();
    let (x, y) = panel_pos(coord, &panel);

    panel.draw(x, y);
}

fn text_pos(coord: Coord, text: &str, size: u16, y_offset: f32) -> (f32, f32) {
    let size = measure_text(text, UI_FONT.get(), size, 1.0);
    let x = ORIGIN.x + coord.x as f32 * TILE_SIZE + TILE_SIZE / 2.0 - size.width / 2.0;

    let y = ORIGIN.y + coord.y as f32 * TILE_SIZE + size.offset_y + TILE_SIZE / 2.0
        - size.height / 2.0
        + y_offset;

    (x, y)
}

fn panel_pos(coord: Coord, panel: &Panel) -> (f32, f32) {
    let x = ORIGIN.x + coord.x as f32 * TILE_SIZE + TILE_SIZE / 2.0 - panel.get_width() / 2.0;
    let y = ORIGIN.y + coord.y as f32 * TILE_SIZE + TILE_SIZE / 2.0 - panel.get_height() / 2.0;
    (x, y)
}
