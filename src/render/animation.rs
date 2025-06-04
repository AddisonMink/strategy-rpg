use crate::{engine::asset::UI_FONT, prelude::*};

pub fn draw_animation(game: &Game, animation: &Animation) {
    let progress = animation.elapsed / animation.duration;

    match &animation.kind {
        AnimationKind::Number {
            coord,
            value,
            color,
        } => grid::draw_text(*coord, &value.to_string(), *color),
        AnimationKind::Meter { .. } => {}
        AnimationKind::Message { coord, text, color } => grid::draw_text(*coord, text, *color),
        AnimationKind::PanelMessage { coord, title, text } => {}
    }
}
