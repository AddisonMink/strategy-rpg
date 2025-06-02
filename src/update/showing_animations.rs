use std::collections::VecDeque;

use crate::prelude::*;

use super::ending_turn::to_ending_turn;

pub fn to_showing_animations(
    game: &mut Game,
    animations: VecDeque<Animation>,
    effects: VecDeque<Effect>,
) {
    game.state = GameState::ShowingAnimations {
        animations,
        effects,
    };
}

pub fn update_showing_animations(game: &mut Game, delta_time: f32) {
    let GameState::ShowingAnimations {
        animations,
        effects,
    } = &mut game.state
    else {
        return;
    };

    if let Some(animation) = animations.front_mut() {
        animation.elapsed += delta_time;
        if animation.elapsed >= animation.duration {
            animations.pop_front();
        }
    }

    if animations.is_empty() && effects.is_empty() {
        to_ending_turn(game);
    }
}
