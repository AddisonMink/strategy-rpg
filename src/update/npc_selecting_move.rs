use crate::prelude::*;

pub fn to_npc_selecting_move(game: &mut Game) {
    game.state = GameState::NpcSelectingMove;
}
