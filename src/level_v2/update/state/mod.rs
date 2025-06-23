use super::super::*;

mod ending_turn;
mod selecting_move;

pub fn update_state(world: &mut World, state: &mut State) {
    match state {
        State::Starting => {
            world.effects.push_front(Effect::UpdateNpcVision);
            world.effects.push_front(Effect::UpdatePlayerVision);
            world.effects.push_front(Effect::UpdateLightGrid);
            selecting_move::transition(world, state);
        }
        State::SelectingMove(..) => selecting_move::update(world, state),
        State::ResolvingMove => ending_turn::transition(world, state),
        State::EndingTurn => ending_turn::update(world, state),
    }
}
