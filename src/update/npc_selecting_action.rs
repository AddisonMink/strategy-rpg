use std::collections::VecDeque;

use crate::prelude::*;

use super::executing_effects::to_executing_effects;

pub fn to_npc_selecting_action(game: &mut Game) {
    game.state = GameState::NpcSelectingAction;
}

pub fn update_npc_selecting_action(game: &mut Game) {
    let unit = game.active_unit().unwrap();

    let effects = unit
        .npc_behavior
        .as_ref()
        .and_then(|b| ((b.select_action)(unit, game)))
        .unwrap_or(VecDeque::new());

    to_executing_effects(game, effects);
}
