use crate::prelude::*;

pub fn to_ending_turn(game: &mut Game) {
    game.state = GameState::EndingTurn;
}
