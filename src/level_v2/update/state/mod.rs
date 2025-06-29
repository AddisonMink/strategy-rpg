use super::super::*;

mod ending_turn;
mod panel;
mod selecting_action;
mod selecting_move;
mod selecting_target;

use panel::*;

pub fn update_state(world: &mut World, state: &mut State) {
    match state {
        State::Starting => {
            world.effects.push_front(Effect::UpdateNpcVision);
            world.effects.push_front(Effect::UpdatePlayerVision);
            world.effects.push_front(Effect::UpdateLightGrid);
            selecting_move::transition(world, state);
        }
        State::SelectingMove(..) => selecting_move::update(world, state),
        State::ResolvingMove => selecting_action::transition(world, state),
        State::SelectingAction(..) => selecting_action::update(world, state),
        State::SelectingEnemyTarget(..) => selecting_target::update_single_enemy(world, state),
        State::ResolvingAction => ending_turn::transition(world, state),
        State::EndingTurn => ending_turn::update(world, state),
        State::Failure => {
            if mouse_clicked() {
                *state = State::Ending(LevelResult::Restart);
            }
        }
        State::Success => {
            if mouse_clicked() {
                *state = State::Ending(LevelResult::Restart);
            }
        }
        State::Ending(result_code) => {}
    };
}
