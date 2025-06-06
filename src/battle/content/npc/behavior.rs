use super::*;
use std::collections::VecDeque;

pub fn select_action_noop(_battle: &Battle, _unit: &Unit) -> Option<VecDeque<Effect>> {
    None
}

pub fn nearest_player(battle: &Battle, unit: &Unit) -> Option<UnitId> {
    battle
        .unit_iter()
        .filter(|u| u.side == Side::Player)
        .min_by_key(|&player_unit| unit.coord.manhattan_distance(player_unit.coord))
        .map(|player_unit| player_unit.id)
}

pub fn find_path_to_adjacent(battle: &Battle, unit: &Unit, target: Coord) -> VecDeque<Coord> {
    let accept = |c: Coord| battle.map.tile(c).walkable && battle.unit_at(c).is_none();
    let goal = |c: Coord| c.manhattan_distance(target) <= 1;
    let mut path = algorithm::breadth_first_search(unit.coord, accept, goal);
    path.truncate(unit.movement as usize);
    path
}

pub fn chase_nearest_player(battle: &Battle, unit: &Unit) -> Option<VecDeque<Coord>> {
    let target_id = nearest_player(battle, unit)?;
    let target = battle.unit(target_id)?;
    let path = find_path_to_adjacent(battle, unit, target.coord);
    if path.is_empty() { None } else { Some(path) }
}
