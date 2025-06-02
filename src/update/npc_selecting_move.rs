use super::npc_executing_move::to_npc_executing_move;
use crate::prelude::*;
use std::collections::VecDeque;

pub fn to_npc_selecting_move(game: &mut Game) {
    game.state = GameState::NpcSelectingMove;
}

pub fn update_npc_selecting_move(game: &mut Game) {
    let unit = game.active_unit().unwrap();

    let path = unit
        .npc_behavior
        .as_ref()
        .and_then(|b| (b.select_move)(unit, game))
        .unwrap_or(VecDeque::new());

    to_npc_executing_move(game, path);
}
