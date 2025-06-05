use super::executing_effects;
use crate::engine::*;
use std::collections::HashSet;

use super::model::*;

pub fn transition(battle: &mut Battle, action: Action) {
    let unit = battle.active_unit().expect("No active unit");

    let targets = match action.range {
        Range::SingleUnit { min, max } => Range::single_unit_targets(battle, unit.coord, min, max),
    };

    if targets.len() == 1 {
        let target_id = targets.iter().next().expect("No target found");
        let effects = action.compile_single_unit_target_effects(*target_id);
        executing_effects::transition(battle, effects);
        return;
    } else {
        battle.state = BattleState::SelectingSingleUnitTarget {
            action,
            targets: HashSet::from_iter(targets),
            selected_target: None,
        };
    }
}

pub fn update(battle: &mut Battle) {
    let BattleState::SelectingSingleUnitTarget {
        action,
        targets,
        selected_target,
    } = &battle.state
    else {
        return;
    };

    if input::mouse_clicked() && selected_target.is_some() {
        let target_id = selected_target.unwrap();
        let effects = action.compile_single_unit_target_effects(target_id);
        executing_effects::transition(battle, effects);
    } else {
        let target_opt = grid::mouse_coord()
            .and_then(|c| battle.unit_at(c))
            .filter(|u| targets.contains(&u.id))
            .map(|u| u.id);

        if let BattleState::SelectingSingleUnitTarget {
            selected_target, ..
        } = &mut battle.state
        {
            *selected_target = target_opt;
        }
    }
}
