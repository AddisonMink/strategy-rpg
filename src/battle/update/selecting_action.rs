use super::model::*;
use super::selecting_move;
use crate::battle::update::selecting_target;
use crate::engine::*;

// There's BAD stuff in here that violates the separation of concerns!

const ACTION_MENU_OFFSET_X: f32 = 360.0;
const ACTION_MENU_OFFSET_Y: f32 = 10.0;

pub fn transition(battle: &mut Battle) {
    let unit = battle.active_unit().expect("No active unit.");

    let valid_actions: Vec<Action> = unit
        .actions()
        .iter()
        .filter(|a| a.has_valid_targets(battle, unit.id, unit.coord))
        .cloned()
        .collect();

    if valid_actions.is_empty() {
        selecting_move::transition(battle);
    } else {
        let panel = build_panel(&valid_actions, None);
        battle.state = BattleState::SelectingAction {
            actions: valid_actions,
            selected_index: None,
            panel,
        };
    }
}

pub fn update(battle: &mut Battle) {
    let BattleState::SelectingAction {
        actions,
        panel,
        selected_index,
    } = &battle.state
    else {
        return;
    };

    if input::mouse_clicked() && selected_index.is_some() {
        let action = actions[selected_index.unwrap()];
        selecting_target::transition(battle, action);
    } else if let Some(num) = input::number_pressed() {
        if let Some(action) = actions.get(num - 1) {
            selecting_target::transition(battle, action.clone());
        }
    } else if input::mouse_clicked() {
        let (mouse_x, mouse_y) = input::mouse_pos();
        let relative_x = mouse_x - ACTION_MENU_OFFSET_X;
        let relative_y = mouse_y - ACTION_MENU_OFFSET_Y;
        let new_selected_index = panel.get_selected_line((relative_x, relative_y));

        let BattleState::SelectingAction {
            actions,
            selected_index,
            panel,
        } = &mut battle.state
        else {
            return;
        };

        *selected_index = new_selected_index;
        *panel = build_panel(actions, new_selected_index);
    } else {
        let (mouse_x, mouse_y) = input::mouse_pos();
        let relative_x = mouse_x - ACTION_MENU_OFFSET_X;
        let relative_y = mouse_y - ACTION_MENU_OFFSET_Y;
        let new_selected_index = panel.get_selected_line((relative_x, relative_y));

        let BattleState::SelectingAction {
            actions: valid_actions,
            selected_index,
            panel,
        } = &mut battle.state
        else {
            return;
        };

        *selected_index = new_selected_index;
        *panel = build_panel(valid_actions, *selected_index);
    }
}

fn build_panel(actions: &Vec<Action>, selected_index: Option<usize>) -> Panel {
    let mut panel = Panel::builder("ACTIONS", WHITE).min_width(200.0);
    for (i, action) in actions.iter().enumerate() {
        let alpha = if selected_index == Some(i) { 1.0 } else { 0.5 };
        let str = format!("{}: {}", i + 1, action.name);
        panel = panel.line(str, WHITE.with_alpha(alpha));
    }
    panel.build()
}
