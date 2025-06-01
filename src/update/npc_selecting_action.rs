use crate::prelude::*;

pub fn to_npc_selecting_action(game: &mut Game) {
    game.state = GameState::NpcSelectingAction;
}
