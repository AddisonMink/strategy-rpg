use super::action;
use super::*;
use macroquad::prelude::trace;

pub fn update(level: &mut Level) {
    let LevelState::SelectingSingleUnitTarget {
        action,
        targets,
        selected_target,
    } = &mut level.state
    else {
        return;
    };

    let mouse_coord = grid::mouse_coord().filter(|c| targets.contains_key(c));
    if mouse_coord.is_some() && input::mouse_clicked() {
        let coord = mouse_coord.unwrap();
        let target = targets.get(&coord).cloned().unwrap();
        let effects = action::compile_single_unit_action(action, target);
        level.effect_queue.extend(effects);
        level.state = LevelState::ResolvingAction;
    } else if let Some(coord) = mouse_coord {
        *selected_target = Some(coord);
    }
}
