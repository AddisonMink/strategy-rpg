use macroquad::prelude::trace;

use super::*;

pub fn draw_animation(battle: &Battle, animation: &Animation) {
    let progress = animation.timer.progress();
    match animation.kind {
        AnimationKind::Number {
            coord,
            value,
            color,
        } => {
            let offset_y = -(progress * grid::TILE_SIZE / 2.0);
            let alpha = 1.0 - progress;
            grid::draw_text_with_offset(
                coord,
                &value.to_string(),
                color.with_alpha(alpha),
                (0.0, offset_y),
            );
        }
        AnimationKind::Attack { unit_id, direction } => {
            let Some(unit) = battle.unit(unit_id) else {
                return;
            };

            let t = 0.25 * (std::f32::consts::PI * progress).sin();
            let origin = unit.coord;
            let target = unit.coord.shift(direction);
            let grid_x = (origin.x as f32 * (1.0 - t) + target.x as f32 * t) as f32;
            let grid_y = (origin.y as f32 * (1.0 - t) + target.y as f32 * t) as f32;
            let offset_x = (grid_x - origin.x as f32) * grid::TILE_SIZE;
            let offset_y = (grid_y - origin.y as f32) * grid::TILE_SIZE;
            grid::draw_glyph_with_offset(unit.coord, unit.glyph, (offset_x, offset_y));
        }
        AnimationKind::PanelMessage {
            coord,
            title,
            title_color,
            message,
            message_color,
        } => {
            let panel = Panel::builder(title.to_string(), title_color)
                .line(message.to_string(), message_color)
                .build();

            grid::draw_panel(&panel, coord);
        }
        AnimationKind::Message { coord, text, color } => {
            let offset_y = -(progress * grid::TILE_SIZE / 2.0);
            let alpha = 1.0 - progress;
            grid::draw_text_with_offset(
                coord,
                &text.to_string(),
                color.with_alpha(alpha),
                (0.0, offset_y),
            );
        }
    }
}
