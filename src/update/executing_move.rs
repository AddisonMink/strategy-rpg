use crate::prelude::*;

pub fn to_executing_move(game: &mut Game, next_coord: Coord, moves_left: u16) {
    game.state = GameState::ExecutingMove {
        next_coord,
        moves_left,
    };
}
