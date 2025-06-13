pub mod selecting_action;
pub mod selecting_move;
pub mod selecting_single_unit_target;
pub mod selecting_target;

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
        LevelState::ResolvingAction => selecting_move::transition(level),
        _ => {}
    }
}
