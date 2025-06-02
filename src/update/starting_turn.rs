use crate::prelude::*;

use super::{npc_selecting_move::to_npc_selecting_move, selecting_move::to_selecting_move};

const TURN_START_DURATION: f32 = 0.5;

pub fn to_starting_turn(game: &mut Game) {
    game.state = GameState::StartingTurn {
        time: TURN_START_DURATION,
    };
}

pub fn update_starting_turn(game: &mut Game, delta_time: f32) {
    let GameState::StartingTurn { time } = &mut game.state else {
        return;
    };

    *time -= delta_time;
    if *time <= 0.0 {
        let unit = game.active_unit().unwrap();
        if unit.is_player {
            to_selecting_move(game);
        } else {
            to_npc_selecting_move(game);
        }
    }
}
