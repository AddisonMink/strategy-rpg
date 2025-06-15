use super::action;
use super::*;

pub fn update(level: &mut Level) {
    let LevelState::SelectingSingleUnitTarget {
        action, targets, ..
    } = &level.state
    else {
        return;
    };

    let mouse_coord = grid::mouse_coord().filter(|c| targets.contains_key(c));
    if mouse_coord.is_some() && input::mouse_clicked() {
        let entity = level.turn_queue.front().cloned().unwrap();
        let coord = mouse_coord.unwrap();
        let target = targets.get(&coord).cloned().unwrap();
        let effects = action::compile_single_unit_action(level, &action.action, entity, target);
        level.effect_queue.extend(effects);
        level.state = LevelState::ResolvingAction;
    } else if let Some(coord) = mouse_coord {
        level.state = LevelState::SelectingSingleUnitTarget {
            action: action.clone(),
            targets: targets.clone(),
            selected_target: Some(coord),
        };
    }
}
