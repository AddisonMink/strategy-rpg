use std::collections::HashSet;

use super::model::*;

pub fn transition(battle: &mut Battle, action: Action) {
    let unit = battle.active_unit().expect("No active unit");

    let targets = match action.range {
        Range::SingleUnit { min, max } => Range::single_unit_targets(battle, unit.coord, min, max),
    };

    battle.state = BattleState::SelectingSingleUnitTarget {
        action,
        targets: HashSet::from_iter(targets),
    };
}
