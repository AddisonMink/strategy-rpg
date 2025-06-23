use super::super::*;
use crate::constants::{UI_ORIGIN, UI_WIDTH};
use crate::engine::coord;
use crate::engine::input::{cancel_pressed, mouse_clicked};
use crate::util::*;

pub fn transition(world: &mut World, state: &mut State) {
    let Some(unit) = world.active_unit() else {
        panic!("No active unit to select move for");
    };

    let mut valid_moves = flood_fill(unit.coord, unit.data().movement, |coord: Coord| {
        world.valid_move(coord)
    });
    valid_moves.remove(&unit.coord);

    let cancel_button = Button::builder("Cancel")
        .min_size(UI_WIDTH, 0.0)
        .position(UI_ORIGIN.0, UI_ORIGIN.1)
        .build();

    let selecting_mvoe = SelectingMove {
        valid_moves,
        path: None,
        cancel_button,
        unit_description_opt: None,
        tile_description_opt: None,
    };

    *state = State::SelectingMove(selecting_mvoe);
}

pub fn update(world: &mut World, state: &mut State) {
    let State::SelectingMove(selecting_move) = state else {
        panic!("Expected SelectingMove state");
    };

    let Some(unit) = world.active_unit() else {
        panic!("No active unit to select move for");
    };

    let coord_opt = grid::mouse_coord().filter(|c| selecting_move.valid_moves.contains(c));

    if selecting_move.cancel_button.is_clicked() || cancel_pressed() {
    } else if selecting_move.path.is_some() && mouse_clicked() {
    } else if coord_opt.is_some() {
        let coord = coord_opt.unwrap();
        let accept = |c: Coord| world.valid_move(c);
        let goal = |c: Coord| c == coord;
        let mut path = breadth_first_search(unit.coord, accept, goal);
        path.truncate(unit.data().movement as usize);
        selecting_move.path = (!path.is_empty()).then_some(path);
    } else if coord_opt.is_none() {
        selecting_move.path = None;
    }
}
