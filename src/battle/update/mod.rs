use super::model;
use super::model::*;

mod ending_turn;
mod executing_effects;
mod executing_move;
mod selecting_action;
mod selecting_move;
mod selecting_target;

pub fn update_battle(battle: &mut Battle, delta_time: f32) {
    match &battle.state {
        BattleState::Starting => selecting_move::transition(battle),
        BattleState::SelectingMove { .. } => selecting_move::update(battle),
        BattleState::ExecutingMove { .. } => executing_move::update(battle, delta_time),
        BattleState::SelectingAction { .. } => selecting_action::update(battle),
        BattleState::SelectingSingleUnitTarget { .. } => selecting_target::update(battle),
        BattleState::ExecutingEffects { .. } => executing_effects::update(battle, delta_time),
        BattleState::EndingTurn => ending_turn::update(battle),
    }
}
