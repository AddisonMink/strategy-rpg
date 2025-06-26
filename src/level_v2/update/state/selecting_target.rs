use panel::*;
use std::collections::{HashMap, HashSet};

use crate::constants::{PADDING, UI_ORIGIN};

use super::*;

pub fn transition(world: &mut World, state: &mut State, action: Action) {
    let unit = world.active_unit().expect("No active unit found");

    let Some(targets) = action.find_targets(&world, unit) else {
        *state = State::EndingTurn;
        return;
    };

    match targets {
        ActionTargets::EnemyTargets(targets) => {
            transition_single_enemy(world, state, action, targets)
        }
    }
}

fn transition_single_enemy(
    world: &mut World,
    state: &mut State,
    action: Action,
    targets: HashSet<UnitId>,
) {
    let targets: HashMap<Coord, UnitId> = targets
        .into_iter()
        .filter_map(|id| world.unit(id).map(|unit| (unit.coord, id)))
        .collect();

    let mut y = UI_ORIGIN.1;
    let cancel_button = make_cancel_button(&mut y);
    let action_description = make_action_description_panel(&action, &mut y);

    *state = State::SelectingEnemyTarget(SelectingEnemyTarget {
        action,
        targets,
        selected_target: None,
        cancel_button,
        action_description,
        unit_description_opt: None,
        tile_description_opt: None,
    })
}

pub fn update_single_enemy(world: &mut World, state: &mut State) {
    let State::SelectingEnemyTarget(selecting) = state else {
        panic!("Expected SelectingEnemyTarget state");
    };

    let mouse_coord_opt = grid::mouse_coord();
    let target_coord = mouse_coord_opt.filter(|c| selecting.targets.contains_key(c));
    let selected_target_coord = target_coord.filter(|_| mouse_clicked());

    let (tile_description_opt, unit_description_opt) = tile_and_unit_panels(
        world,
        mouse_coord_opt,
        selecting.action_description.get_y2() + PADDING,
    );

    selecting.selected_target = target_coord;
    selecting.tile_description_opt = tile_description_opt;
    selecting.unit_description_opt = unit_description_opt;

    // If cancel button is clicked or cancel is pressed, end the turn.
    if selecting.cancel_button.is_clicked() || cancel_pressed() {
        *state = State::EndingTurn;
    }
    // If a target is selected, resolve the action.
    else if let Some(coord) = selected_target_coord {
        let target_id = selecting.targets[&coord];
        *state = State::ResolvingAction;
    }
}

fn tile_and_unit_panels(
    world: &World,
    mouse_coord: Option<Coord>,
    y: f32,
) -> (Option<Panel>, Option<Panel>) {
    let unit_id = world.active_unit().expect("No active unit found").id();

    let tile_opt = mouse_coord
        .filter(|c| world.unit_can_see_tile(unit_id, *c))
        .map(|c| world.map.tile(c));

    let unit_opt = mouse_coord
        .and_then(|c| world.unit_at(c))
        .filter(|u| world.unit_can_see_unit(unit_id, u.id()));

    let mut _y = y;

    let tile_description_opt = tile_opt.map(|tile| make_tile_description_panel(tile, &mut _y));
    let unit_description_opt = unit_opt.map(|unit| make_unit_description_panel(unit, &mut _y));
    (unit_description_opt, tile_description_opt)
}
