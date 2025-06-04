use super::model;
use super::model::*;

mod executing_move;
mod selecting_move;

pub fn update_battle(battle: &mut Battle, delta_time: f32) {
    match &battle.state {
        BattleState::Starting => selecting_move::transition(battle),
        BattleState::SelectingMove { .. } => selecting_move::update(battle),
        BattleState::ExecutingMove { .. } => executing_move::update(battle, delta_time),
    }
}
