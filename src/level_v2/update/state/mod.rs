use super::super::*;

mod selecting_move;

pub fn update_state(world: &mut World, state: &mut State, delta_time: f32) {
    match state {
        State::Starting(..) => {
            world.effects.push_front(Effect::UpdatePlayerVision);
            world.effects.push_front(Effect::UpdateLightGrid);
            selecting_move::transition(world, state);
        }
        State::SelectingMove(..) => selecting_move::update(world, state),
    }
}
