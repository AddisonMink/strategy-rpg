use crate::prelude::*;

use super::{executing_move::to_executing_move, selecting_action::to_selecting_action};

pub fn to_selecting_move(game: &mut Game) {
    let unit = game.active_unit().unwrap();
    game.state = GameState::SelectingMove {
        moves_left: unit.movement,
    };
}

pub fn to_selecting_move_ex(game: &mut Game, moves_left: u16) {
    game.state = GameState::SelectingMove { moves_left };
}

pub fn update_selecting_move(game: &mut Game) {
    let GameState::SelectingMove { moves_left } = game.state else {
        return;
    };

    if input::pressed_cancel() {
        to_selecting_action(game);
    } else if let Some(dir) = input::pressed_direction() {
        let coord = game.active_unit().unwrap().coord;
        let next_coord = coord.shift(dir);
        let walkable = game.map.walkable(next_coord);
        let unoccupied = game.unit_at(next_coord).is_none();

        if walkable && unoccupied {
            to_executing_move(game, next_coord, moves_left - 1);
        }
    }
}
