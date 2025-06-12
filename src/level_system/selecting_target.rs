use std::collections::HashMap;

use macroquad::prelude::trace;

use super::action;
use crate::level_model::*;

pub fn transition(level: &mut Level, action: Action) {
    match action.range {
        Range::SelfRange => {}
        Range::SingleUnit { min, max } => single_unit_action(level, action, min, max),
    }
}

fn single_unit_action(level: &mut Level, action: Action, min: u16, max: u16) {
    let entity = level.turn_queue.front().unwrap();
    let origin = level.positions.get(entity).unwrap().coord;
    let targets = action::single_unit_range_targets(level, *entity, origin, min, max);

    if targets.is_empty() {
        trace!("No valid targets found for action: {:?}", action);
    } else {
        let mut ts = HashMap::new();
        for target in targets {
            let coord = level.positions.get(&target).unwrap().coord;
            ts.insert(coord, target);
        }
        level.state = LevelState::SelectingSingleUnitTarget {
            action: action.clone(),
            targets: ts,
            selected_target: None,
        };
    }
}
