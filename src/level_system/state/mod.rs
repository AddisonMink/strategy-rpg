mod ending_turn;
mod selecting_action;
mod selecting_move;
mod selecting_single_unit_target;
mod selecting_target;

use macroquad::prelude::trace;

use super::action;
use crate::engine::*;
use crate::level_model::*;

pub fn process_state(level: &mut Level) {
    match level.state {
        LevelState::Starting => {
            level.effect_queue.push_back(Effect::UpdateLightGrid);
            level.effect_queue.push_back(Effect::UpdateVisionGrid);
            level.effect_queue.push_back(Effect::UpdateAllNpcVision);
            selecting_move::transition(level);
        }
        LevelState::SelectingMove { .. } => selecting_move::update(level),
        LevelState::ResolvingMove => selecting_action::transition(level),
        LevelState::SelectingAction { .. } => selecting_action::update(level),
        LevelState::SelectingSingleUnitTarget { .. } => selecting_single_unit_target::update(level),
        LevelState::ResolvingAction => ending_turn::transition(level),
        LevelState::EndingTurn => ending_turn::update(level),
        LevelState::Success => {}
    }
}
