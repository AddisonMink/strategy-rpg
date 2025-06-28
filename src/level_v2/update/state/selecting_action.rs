use super::*;
use crate::constants::{PADDING, UI_ORIGIN};
use panel::*;

pub fn transition(world: &mut World, state: &mut State) {
    let unit = world.active_unit().expect("No active unit found");

    if unit.data().side == Side::NPC {
        if let Some(effects) = (unit.behavior().select_action)(world) {
            world.effects.extend(effects);
        }
        *state = State::ResolvingAction;
    } else {
        let actions: Vec<Action> = unit
            .actions()
            .iter()
            .filter(|a| a.find_targets(world, unit).is_some())
            .copied()
            .cloned()
            .collect();

        if actions.is_empty() {
            ending_turn::transition(world, state);
        } else {
            let mut y = UI_ORIGIN.1;
            let cancel_button = make_cancel_button(&mut y);
            let action_list = make_action_list_panel(&actions, &mut y);

            *state = State::SelectingAction(SelectingAction {
                actions,
                cancel_button,
                action_list,
                unit_description_opt: None,
                tile_description_opt: None,
                action_description_opt: None,
            })
        }
    }
}

pub fn update(world: &mut World, state: &mut State) {
    let State::SelectingAction(selecting_action) = state else {
        panic!("Expected state to be SelectingAction, found {:?}", state);
    };

    let unit_id = world.active_unit_id().expect("No active unit found");
    update_panels(world, selecting_action, unit_id);

    // If cancel button is clicked or cancel is pressed, end the turn.
    if selecting_action.cancel_button.is_clicked() || cancel_pressed() {
        ending_turn::transition(world, state);
    }
    // If an action is clicked, select it.
    else if let Some(index) = selecting_action.action_list.clicked_index() {
        if let Some(action) = selecting_action.actions.get(index).cloned() {
            selecting_target::transition(world, state, action.clone());
        }
    }
    // If a number is pressed, select the corresponding action.
    else if let Some(index) = number_pressed() {
        if let Some(action) = selecting_action.actions.get(index - 1).cloned() {
            selecting_target::transition(world, state, action.clone());
        }
    }
}

fn update_panels(world: &mut World, selecting_action: &mut SelectingAction, unit_id: UnitId) {
    let mouse_coord = grid::mouse_coord();

    let tile_opt = mouse_coord
        .filter(|c| world.unit_can_see_tile(unit_id, *c))
        .map(|c| world.map.tile(c));

    let unit_opt = mouse_coord
        .and_then(|c| world.unit_at(c))
        .filter(|u| world.unit_can_see_unit(unit_id, u.id()));

    let action_opt = selecting_action
        .action_list
        .selected_index()
        .and_then(|i| selecting_action.actions.get(i));

    let mut y = selecting_action.action_list.get_y2() + PADDING;
    selecting_action.tile_description_opt =
        tile_opt.map(|t| make_tile_description_panel(t, &mut y));
    selecting_action.unit_description_opt =
        unit_opt.map(|u| make_unit_description_panel(u, &mut y));
    selecting_action.action_description_opt =
        action_opt.map(|a| make_action_description_panel(a, &mut y));
}
