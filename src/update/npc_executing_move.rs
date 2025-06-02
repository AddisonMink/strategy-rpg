use std::collections::VecDeque;

use crate::prelude::*;

use super::npc_selecting_action::to_npc_selecting_action;

const NPC_MOVE_DURATION: f32 = 0.2;

pub fn to_npc_executing_move(game: &mut Game, path: VecDeque<Coord>) {
    game.state = GameState::NpcExecutingMove { path, time: 0.0 };
}

pub fn update_npc_executing_move(game: &mut Game, delta_time: f32) {
    let GameState::NpcExecutingMove { path, time } = &mut game.state else {
        return;
    };

    *time -= delta_time;

    if path.is_empty() {
        to_npc_selecting_action(game);
    } else if *time <= 0.0 {
        *time = NPC_MOVE_DURATION;
        
        let next_coord = path.pop_front().unwrap();
        let unit = game.active_unit_mut().unwrap();

        unit.coord = next_coord;

        if unit.light.is_some() {
            game.light_grid = LightGrid::new(game);
        }
    }
}
