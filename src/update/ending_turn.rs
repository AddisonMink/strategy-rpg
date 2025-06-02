use crate::prelude::*;

use super::starting_turn::to_starting_turn;

pub fn to_ending_turn(game: &mut Game) {
    game.state = GameState::EndingTurn;
}

pub fn update_ending_turn(game: &mut Game) {
    game.next_turn();
    to_starting_turn(game);
}
