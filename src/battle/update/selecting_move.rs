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
    let mouse_coord_opt = grid::mouse_coord();
    let (valid_moves, path) = unpack(battle);

    // If mouse is clicked on the origin, skip move.
    if input::mouse_clicked() && mouse_coord_opt == Some(origin) || input::cancel_pressed() {
        executing_move::transition(battle, VecDeque::new());
    }
    // If mouse is clicked on a valid move, execute the move.
    else if input::mouse_clicked() && path.is_some() {
        let final_path = path.as_ref().unwrap().clone();
        executing_move::transition(battle, final_path);
    }
    // If mouse is hovering over a valid move, update the path.
    else if mouse_coord_opt.is_some() && valid_moves.contains(&mouse_coord_opt.unwrap()) {
        let accept = |coord: Coord| valid_moves.contains(&coord);
        let goal = |coord: Coord| coord == mouse_coord_opt.unwrap();
        let new_path = algorithm::breadth_first_search(origin, accept, goal);
        *path = Some(new_path);
    }
    // If mouse is not hovering over a valid move, clear the path.
    else if mouse_coord_opt.is_none() || !valid_moves.contains(&mouse_coord_opt.unwrap()) {
        *path = None;
    }
}

fn unpack(battle: &mut Battle) -> (&HashSet<Coord>, &mut Option<VecDeque<Coord>>) {
    if let BattleState::SelectingMove { valid_moves, path } = &mut battle.state {
        (valid_moves, path)
    } else {
        panic!("Cannot unpack in current state.");
    }
}
