use macroquad::prelude::trace;

use super::action;
use crate::engine::*;
use crate::level_model::*;
use crate::level_render::INFO_PANEL_ORIGIN;
use crate::level_render::INFO_PANEL_WIDTH;

pub fn transition(level: &mut Level) {
    let actions = vec![Action::ATTACK, Action::WAIT];

    level.state = LevelState::SelectingAction {
        actions: vec![Action::ATTACK, Action::WAIT],
        panel: make_action_list_panel(&actions, None),
        selected_action: None,
        target_coords: None,
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

    if let Some(selected_line) = panel.get_selected_line(relative_mouse_pos) {
        let action = &actions[selected_line];
        if input::mouse_clicked() {
        } else {
            let entity = level.turn_queue.front().unwrap();
            let origin = level.positions.get(entity).unwrap().coord;
            let new_target_coords = action::find_target_coords(level, origin, action);
            level.state = LevelState::SelectingAction {
                actions: actions.to_vec(),
                panel: make_action_list_panel(actions, Some(selected_line)),
                selected_action: Some(action.clone()),
                target_coords: Some(new_target_coords),
            };
        }
    } else {
        level.state = LevelState::SelectingAction {
            actions: actions.to_vec(),
            panel: make_action_list_panel(actions, None),
            selected_action: None,
            target_coords: None,
        };
    }
}

fn make_action_list_panel(actions: &[Action], selected_index: Option<usize>) -> Panel {
    let mut panel = Panel::builder("ACTIONS", WHITE).min_width(INFO_PANEL_WIDTH);
    for (i, action) in actions.iter().enumerate() {
        let selected = selected_index.map_or(false, |index| index == i);
        let color = if selected { WHITE } else { GRAY };
        let text = format!("{}: {}", i + 1, action.name);
        panel = panel.line(text, color);
    }
    panel.build()
}
