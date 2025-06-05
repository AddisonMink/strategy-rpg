use macroquad::prelude::trace;

use super::model::*;
use std::collections::VecDeque;

pub fn transition(battle: &mut Battle, effects: VecDeque<Effect>) {
    battle.state = BattleState::ExecutingEffects { effects };
}

pub fn update(battle: &mut Battle) {
    trace!("Executing effects update called");
}
