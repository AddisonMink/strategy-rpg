use super::executing_move;
use super::model::*;
use super::selecting_move;

pub fn transition(battle: &mut Battle) {
    battle.state = BattleState::EndingTurn;
}

pub fn update(battle: &mut Battle) {
    battle.next_turn();
    let unit = battle.active_unit().expect("No active unit");

    match unit.side {
        Side::Player => selecting_move::transition(battle),
        Side::NPC => {
            let path = unit.npc_select_move(battle).unwrap_or_default();
            executing_move::transition(battle, path);
        }
    }
}
