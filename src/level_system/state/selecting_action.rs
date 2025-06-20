use super::action;
use super::*;
use crate::level_render::INFO_PANEL_ORIGIN;
use crate::level_render::INFO_PANEL_WIDTH;

pub fn transition(level: &mut Level) {
    let entity = level.turn_queue.front().unwrap();
    let unit = level.units.get(entity).unwrap();

    match unit.side {
        Side::Player => {
            let actions = action::valid_player_actions(&level);

            if actions.is_empty() {
                level.state = LevelState::ResolvingAction;
            } else {
                let panel = make_action_list_panel(&actions, None);

                level.state = LevelState::SelectingAction {
                    actions,
                    panel,
                    selected_action: None,
                    target_coords: None,
                }
            }
        }
        Side::NPC => {
            let effects = (unit.behavior.select_action)(&level).unwrap_or_default();
            level.effect_queue.extend(effects);
            level.state = LevelState::ResolvingAction;
        }
    }
}

pub fn update(level: &mut Level) {
    let LevelState::SelectingAction { actions, panel, .. } = &level.state else {
        return;
    };

    let mouse_pos = input::mouse_pos();

    let relative_mouse_pos = (
        mouse_pos.0 - INFO_PANEL_ORIGIN.0,
        mouse_pos.1 - INFO_PANEL_ORIGIN.1,
    );

    let selected_line_opt = panel.get_selected_line(relative_mouse_pos);

    let number = input::number_pressed()
        .map(|n| n - 1)
        .filter(|n| *n < actions.len());

    // if mouse is clicked and no action is selected, or cancel is pressed, skip action.
    if (input::mouse_clicked() && selected_line_opt.is_none()) || input::cancel_pressed() {
        level.state = LevelState::ResolvingAction;
    }
    // if number is pressed, select the corresponding action
    else if let Some(selected_index) = number {
        let action = &actions[selected_index];
        selecting_target::transition(level, action.clone());
    }
    // if mouse is clicked, select the action under the mouse cursor
    else if input::mouse_clicked() && selected_line_opt.is_some() {
        let selected_line = selected_line_opt.unwrap();
        let action = &actions[selected_line];
        selecting_target::transition(level, action.clone());
    }
    // if mouse is hovered over an action, highlight it
    else if let Some(selected_line) = selected_line_opt {
        let action = &actions[selected_line];
        let unit = level.active_unit().unwrap();

        let new_target_coords =
            action::find_target_coords(level, unit.entity, unit.coord, &action.action);

        level.state = LevelState::SelectingAction {
            actions: actions.to_vec(),
            panel: make_action_list_panel(actions, Some(selected_line)),
            selected_action: Some(action.clone()),
            target_coords: Some(new_target_coords),
        };
    }
    // if mouse is not hovering over an action, reset the selection
    else {
        level.state = LevelState::SelectingAction {
            actions: actions.to_vec(),
            panel: make_action_list_panel(actions, None),
            selected_action: None,
            target_coords: None,
        };
    }
}

fn make_action_list_panel(actions: &Vec<ItemAction>, selected_index: Option<usize>) -> Panel {
    let mut panel = Panel::builder("ACTIONS", WHITE).min_width(INFO_PANEL_WIDTH);

    for (i, action) in actions.iter().enumerate() {
        let selected = selected_index.map_or(false, |index| index == i);
        let alpha = if selected { 1.0 } else { 0.5 };
        let color = action.item_color.with_alpha(alpha);
        let text = format!("{}: {}", i + 1, action.action.name);
        panel = panel.line(text, color);
    }

    panel.build()
}
