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
    }
}
