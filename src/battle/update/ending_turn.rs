use macroquad::prelude::trace;

use super::model::*;
use super::selecting_move;

pub fn transition(battle: &mut Battle) {
    battle.state = BattleState::EndingTurn;
}

pub fn update(battle: &mut Battle) {
    trace!("Ending turn update");
    battle.next_turn();
    selecting_move::transition(battle);
}
