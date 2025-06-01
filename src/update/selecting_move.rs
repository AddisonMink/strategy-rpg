use crate::prelude::*;

pub fn to_selecting_move(game: &mut Game) {
    let unit = game.active_unit().unwrap();
    game.state = GameState::SelectingMove {
        moves_left: unit.movement,
    };
}

pub fn to_selecting_move_ex(game: &mut Game, moves_left: u16) {
    game.state = GameState::SelectingMove { moves_left };
}
