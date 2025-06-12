use crate::engine::grid::TILE_SIZE;
use crate::engine::*;
use crate::level_model::*;

pub fn draw_animation(level: &Level) {
    let Some(animation) = level.animation_queue.front() else {
        return;
    };

    let progress = animation.timer.progress();

    match &animation.kind {
        AnimationKind::Text { coord, text, color } => {
            let offset_y = -(TILE_SIZE / 2.0) * progress;
            let alpha = 1.0 - progress;
            let color = color.with_alpha(alpha);
            grid::draw_text_with_offset(*coord, text, color, (0.0, offset_y));
        }
    }
}
