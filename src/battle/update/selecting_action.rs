use super::model::*;
use super::selecting_move;

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
        battle.state = BattleState::SelectingAction { valid_actions };
    }
}
