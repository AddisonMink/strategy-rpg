use std::collections::VecDeque;

use crate::prelude::*;

const NPC_MOVE_DURATION: f32 = 0.2;

pub fn to_npc_executing_move(game: &mut Game, path: VecDeque<Coord>) {
    game.state = GameState::NpcExecutingMove { path, time: 0.0 };
}
