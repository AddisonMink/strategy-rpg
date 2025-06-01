use std::collections::VecDeque;

use crate::prelude::*;

pub fn to_executing_effects(game: &mut Game, effects: VecDeque<Effect>) {
    game.state = GameState::ExecutingEffects { effects };
}
