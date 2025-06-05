use std::collections::HashSet;

use crate::engine::grid;

use super::model::*;

pub fn transition(battle: &mut Battle, action: Action) {
    let unit = battle.active_unit().expect("No active unit");

    let targets = match action.range {
        Range::SingleUnit { min, max } => Range::single_unit_targets(battle, unit.coord, min, max),
    };

    battle.state = BattleState::SelectingSingleUnitTarget {
        action,
        targets: HashSet::from_iter(targets),
        selected_target: None,
    };
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
