use crate::prelude::*;

const TURN_START_DURATION: f32 = 0.5;

pub fn to_starting_turn(game: &mut Game) {
    game.state = GameState::StartingTurn {
        time: TURN_START_DURATION,
    };
}
