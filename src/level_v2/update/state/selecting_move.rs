use std::collections::VecDeque;

use super::*;
use crate::constants::{PADDING, UI_ORIGIN};
use crate::engine::input::{cancel_pressed, mouse_clicked};
use crate::util::*;

const MOVE_DURATION: f32 = 0.25;

pub fn transition(world: &mut World, state: &mut State) {
    let Some(unit) = world.active_unit() else {
        panic!("No active unit to select move for");
    };

    if unit.data().side == Side::NPC {
        if let Some(path) = (unit.behavior().select_move)(world) {
            let effects = compile_path(unit.id(), &path);
            world.effects.extend(effects);
        }
        *state = State::ResolvingMove;
    } else {
        let mut valid_moves = flood_fill(unit.coord, unit.data().movement, |coord: Coord| {
            world.valid_move(coord)
        });
        valid_moves.remove(&unit.coord);

        let mut y = UI_ORIGIN.1;
        let cancel_button = make_cancel_button(&mut y);
        let action_preview = make_action_preview_panel(world, None, &mut y);

        let selecting_move = SelectingMove {
            valid_moves,
            path: None,
            cancel_button,
            action_preview,
            unit_description_opt: None,
            tile_description_opt: None,
        };

        *state = State::SelectingMove(selecting_move);
    }
}

pub fn update(world: &mut World, state: &mut State) {
    let State::SelectingMove(selecting_move) = state else {
        panic!("Expected SelectingMove state");
    };

    let coord_opt = grid::mouse_coord();

    if let Some(id) = world.active_unit_id() {
        update_panels(world, selecting_move, id, coord_opt);
    }

    let Some(unit) = world.active_unit() else {
        panic!("No active unit to select move for");
    };

    let valid_coord_opt = coord_opt.filter(|c| selecting_move.valid_moves.contains(c));

    // If cancel button is clicked or cancel is pressed, cancel the move.
    if selecting_move.cancel_button.is_clicked() || cancel_pressed() {
        *state = State::ResolvingMove;
    }
    // If mouse is clicked and a path is selected, execute the move.
    else if selecting_move.path.is_some() && mouse_clicked() {
        let id = unit.id();
        let effects = compile_path(id, &selecting_move.path.as_ref().unwrap());
        world.effects.extend(effects);
        *state = State::ResolvingMove;
    }
    // If mouse is over a valid move, update the path.
    else if valid_coord_opt.is_some() {
        let coord = valid_coord_opt.unwrap();
        let accept = |c: Coord| world.valid_move(c);
        let goal = |c: Coord| c == coord;
        let mut path = breadth_first_search(unit.coord, accept, goal);
        path.truncate(unit.data().movement as usize);
        selecting_move.path = (!path.is_empty()).then_some(path);
    }
    // If mouse is not over a valid move, clear the path.
    else if valid_coord_opt.is_none() {
        selecting_move.path = None;
    }
}

fn update_panels(
    world: &mut World,
    selecting_move: &mut SelectingMove,
    unit_id: UnitId,
    mouse_coord: Option<Coord>,
) {
    let tile_opt = mouse_coord
        .filter(|c| world.unit_can_see_tile(unit_id, *c))
        .map(|c| world.map.tile(c));

    let unit_opt = mouse_coord
        .and_then(|c| world.unit_at(c))
        .filter(|u| world.unit_can_see_unit(unit_id, u.id()));

    let mut y = selecting_move.cancel_button.get_y2() + PADDING;
    selecting_move.action_preview = make_action_preview_panel(world, mouse_coord, &mut y);
    selecting_move.tile_description_opt = tile_opt.map(|t| make_tile_description_panel(t, &mut y));
    selecting_move.unit_description_opt = unit_opt.map(|u| make_unit_description_panel(u, &mut y));
}

fn compile_path(id: UnitId, path: &VecDeque<Coord>) -> VecDeque<Effect> {
    let mut effects = VecDeque::new();

    for coord in path.iter() {
        effects.push_back(Effect::Move { id, coord: *coord });

        effects.push_back(Effect::Sleep {
            duration: MOVE_DURATION,
        });
    }

    if !path.is_empty() {
        effects.pop_back();
    }

    effects
}
