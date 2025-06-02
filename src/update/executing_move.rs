use crate::prelude::*;

use super::{selecting_action::to_selecting_action, selecting_move::to_selecting_move_ex};

pub fn to_executing_move(game: &mut Game, next_coord: Coord, moves_left: u16) {
    game.state = GameState::ExecutingMove {
        next_coord,
        moves_left,
    };
}

pub fn update_executing_move(game: &mut Game) {
    let GameState::ExecutingMove {
        next_coord,
        moves_left,
    } = game.state
    else {
        return;
    };

    let unit = game.active_unit_mut().unwrap();
    unit.coord = next_coord;

    if unit.light.is_some() {
        game.light_grid = LightGrid::new(game);
    }

    if moves_left > 0 {
        to_selecting_move_ex(game, moves_left);
    } else {
        to_selecting_action(game);
    }
}
