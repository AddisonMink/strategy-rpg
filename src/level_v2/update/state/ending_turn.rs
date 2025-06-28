use super::*;

pub fn transition(world: &mut World, state: &mut State) {
    *state = State::EndingTurn;
}

pub fn update(world: &mut World, state: &mut State) {
    world.end_turn();

    if world.player_units_iter().next().is_none() {
        *state = State::Failure;
    } else {
        selecting_move::transition(world, state);
    }
}
