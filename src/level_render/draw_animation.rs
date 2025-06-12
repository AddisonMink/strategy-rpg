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
        AnimationKind::Entity(entity_animation) => draw_entity_animation(
            level,
            entity_animation.entity,
            entity_animation.kind,
            progress,
        ),
    }
}

fn draw_entity_animation(level: &Level, entity: Entity, kind: EntityAnimationKind, progress: f32) {
    if !level.player_vision.entity_visible(entity) {
        return;
    }

    let Some(unit) = level.units.get(&entity) else {
        return;
    };

    let coord = level.positions.get(&entity).unwrap().coord;
    let light_color = level.light_grid.light_color(coord);
    let color = color::mix_color(unit.glyph.color, light_color, 0.5);
    let glyph = Glyph::new(unit.glyph.symbol, color);

    match kind {
        EntityAnimationKind::Attack { direction } => {
            let t = (progress * std::f32::consts::PI).sin() * TILE_SIZE / 2.0;
            let offset = match direction {
                Direction::Up => (0.0, -t),
                Direction::Down => (0.0, t),
                Direction::Left => (-t, 0.0),
                Direction::Right => (t, 0.0),
            };
            grid::draw_glyph_with_offset(coord, glyph, offset);
        }
    }
}
