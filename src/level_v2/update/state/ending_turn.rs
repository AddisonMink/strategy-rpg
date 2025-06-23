use super::*;

pub fn transition(world: &mut World, state: &mut State) {
    *state = State::EndingTurn;
}

pub fn update(world: &mut World, state: &mut State) {
    world.end_turn();
    selecting_move::transition(world, state);
}
