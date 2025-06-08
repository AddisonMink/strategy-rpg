use super::*;
use std::collections::VecDeque;

pub fn find_visible_players(battle: &Battle, unit_id: UnitId) -> impl Iterator<Item = &Unit> {
    let unit = battle.unit(unit_id).expect("Unit should exist");

    battle.unit_player_iter().filter(|&player_unit| {
        let distance = unit.coord.manhattan_distance(player_unit.coord);
        let distance_from_light = battle
            .get_light_grid()
            .distance_from_light(player_unit.coord);
        let line_of_sight = battle
            .map
            .check_line_of_sight(unit.coord, player_unit.coord);
        line_of_sight && (distance <= unit.vision || distance_from_light <= unit.vision)
    })
}

pub fn find_nearest_visible_player(battle: &Battle, unit_id: UnitId) -> Option<&Unit> {
    let unit = battle.unit(unit_id)?;
    find_visible_players(battle, unit_id)
        .min_by_key(|&player_unit| unit.coord.manhattan_distance(player_unit.coord))
}

pub fn find_path_to(battle: &Battle, unit: &Unit, target: Coord) -> VecDeque<Coord> {
    let accept = |c: Coord| battle.map.tile(c).walkable && battle.unit_at(c).is_none();
    let goal = |c: Coord| c == target;
    let mut path = algorithm::breadth_first_search(unit.coord, accept, goal);
    path.truncate(unit.movement as usize);
    path
}

pub fn find_path_to_adjacent(battle: &Battle, unit: &Unit, target: Coord) -> VecDeque<Coord> {
    let accept = |c: Coord| battle.map.tile(c).walkable && battle.unit_at(c).is_none();
    let goal = |c: Coord| c.manhattan_distance(target) <= 1;
    let mut path = algorithm::breadth_first_search(unit.coord, accept, goal);
    path.truncate(unit.movement as usize);
    path
}
