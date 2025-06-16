use super::*;
use macroquad::prelude::trace;
use std::collections::HashMap;

pub fn transition(level: &mut Level, action: ItemAction) {
    match action.action.range {
        Range::SelfRange => self_action(level, action),
        Range::SingleUnit { min, max } => single_unit_action(level, action, min, max),
    }
}

fn self_action(level: &mut Level, action: ItemAction) {
    let entity = level.turn_queue.front().unwrap();
    let effects = action::compile_self_action(level, &action, *entity);
    level.effect_queue.extend(effects);
    level.state = LevelState::ResolvingAction;
}

fn single_unit_action(level: &mut Level, action: ItemAction, min: u16, max: u16) {
    let unit = level.active_unit().unwrap();
    let targets = action::single_unit_range_targets(level, unit.entity, unit.coord, min, max);

    // If there is only 1 target, select it automatically.
    if targets.iter().count() == 1 {
        let target = targets.iter().next().unwrap();
        let effects = action::compile_single_unit_action(level, &action, unit.entity, *target);
        level.effect_queue.extend(effects);
        level.state = LevelState::ResolvingAction;
    } else {
        let mut ts = HashMap::new();
        for target in targets {
            let coord = level.units.get(&target).unwrap().coord;
            ts.insert(coord, target);
        }
        level.state = LevelState::SelectingSingleUnitTarget {
            action: action.clone(),
            targets: ts,
            selected_target: None,
        };
    }
}
