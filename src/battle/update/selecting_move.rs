use super::model::*;
use crate::engine::*;

pub fn transition(battle: &mut Battle) {
    let unit = battle.active_unit().expect("No active unit.");
    let accept = |coord: Coord| battle.map.tile(coord).walkable && battle.unit_at(coord).is_none();
    let mut valid_moves = algorithm::flood_fill(unit.coord, 3, accept);
    valid_moves.remove(&unit.coord);
    battle.state = BattleState::SelectingMove { valid_moves };
}
