use super::executing_move;
use super::model::*;
use crate::engine::*;
use std::collections::{HashSet, VecDeque};

pub fn transition(battle: &mut Battle) {
    let unit = battle.active_unit().expect("No active unit.");
    let accept = |coord: Coord| battle.map.tile(coord).walkable && battle.unit_at(coord).is_none();
    let mut valid_moves = algorithm::flood_fill(unit.coord, unit.movement, accept);
    valid_moves.remove(&unit.coord);
    battle.state = BattleState::SelectingMove {
        valid_moves,
        path: None,
    };
}

pub fn update(battle: &mut Battle) {
    let origin = battle.active_unit().expect("No active unit.").coord;
    let valid_moves = get_valid_moves(battle);

    let Some(end) = grid::mouse_coord() else {
        set_path(battle, VecDeque::new());
        return;
    };

    if !valid_moves.contains(&end) {
        set_path(battle, VecDeque::new());
        return;
    }

    let accept = |coord: Coord| valid_moves.contains(&coord);
    let goal = |coord: Coord| coord == end;
    let new_path = algorithm::breadth_first_search(origin, accept, goal);

    if !new_path.is_empty() {
        if input::mouse_clicked() {
            executing_move::transition(battle, new_path.clone());
        } else {
            set_path(battle, new_path);
        }
    }
}

pub fn get_valid_moves(battle: &Battle) -> &HashSet<Coord> {
    if let BattleState::SelectingMove { valid_moves, .. } = &battle.state {
        valid_moves
    } else {
        panic!("Cannot get valid moves in current state.");
    }
}

pub fn set_path(battle: &mut Battle, path: VecDeque<Coord>) {
    if let BattleState::SelectingMove { path: p, .. } = &mut battle.state {
        *p = Some(path);
    } else {
        panic!("Cannot set path in current state.");
    }
}
