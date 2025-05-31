use std::collections::VecDeque;

use crate::prelude::*;

pub fn coord_is_visible(game: &Game, origin: Coord, coord: Coord, vision: u16) -> bool {
    let distance = origin.manhattan_distance(coord);
    let distance_from_light = game.light_grid.distance_from_light(coord);

    game.map.check_line_of_sight(origin, coord)
        && (distance <= vision || distance_from_light <= vision)
}

pub fn find_nearest_visible_player(game: &Game, origin: Coord, vision: u16) -> Option<&Unit> {
    let mut nearest_player = None;
    let mut min_distance = u16::MAX;

    for unit in game.unit_players_iter() {
        if coord_is_visible(game, origin, unit.coord, vision) {
            let distance = origin.manhattan_distance(unit.coord);
            if distance < min_distance {
                min_distance = distance;
                nearest_player = Some(unit);
            }
        }
    }

    nearest_player
}

pub fn find_path_to_adjacent(game: &Game, origin: Coord, target: Coord) -> VecDeque<Coord> {
    let accept = |coord: Coord| game.map.walkable(coord) && game.unit_at(coord).is_none();
    let goal = |coord: Coord| coord.manhattan_distance(target) <= 1;
    algorithm::breadth_first_search(origin, accept, goal)
}
