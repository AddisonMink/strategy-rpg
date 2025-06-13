use std::collections::VecDeque;

use crate::engine::*;
use crate::level_model::*;

/// Standard move behavior.
/// If there is a visible player, it moves towards the nearest one.
/// If no visible player is found, it moves towards the last seen player position.
/// If no players are visible or last seen, it does not move.
pub fn standard_move(level: &Level) -> VecDeque<Coord> {
    let me = level.turn_queue.front().cloned().unwrap();
    let pos = level.positions.get(&me).unwrap();
    let unit = level.units.get(&me).unwrap();
    let memory = level.vision_memory.get(&me).unwrap();
    let nearest_visible_player = find_nearest_visible_player(level);

    let mut path = if let Some(target) = nearest_visible_player {
        find_path_to_adjacent(level, pos.coord, target.coord)
    } else if let Some((_, player_pos)) = memory.last_seen_player {
        find_path_to_adjacent(level, pos.coord, player_pos)
    } else {
        VecDeque::new() // No valid target found
    };

    path.truncate(unit.movement as usize);
    path
}

pub fn standard_action(level: &Level) -> VecDeque<Effect> {
    let mut effects = VecDeque::new();
    let entity = level.turn_queue.front().cloned().unwrap();
    let pos = level.positions.get(&entity).unwrap();

    let player_opt =
        find_nearest_visible_player(level).filter(|p| p.coord.manhattan_distance(pos.coord) == 1);

    if let Some(player_pos) = player_opt {
        let direction = pos.coord.direction_to(player_pos.coord).unwrap();
        let entity = level.turn_queue.front().cloned().unwrap();
        effects.push_back(Effect::Animation {
            animation: Animation::attack(entity, direction),
        });
        effects.push_back(Effect::Damage {
            entity: player_pos.entity,
            min: 0,
            max: 3,
        });
    }

    effects
}

pub fn find_nearest_visible_player(level: &Level) -> Option<&Position> {
    let entity = level.turn_queue.front().cloned().unwrap();
    let memory = level.vision_memory.get(&entity).unwrap();
    let pos = level.positions.get(&entity).unwrap();

    memory
        .visible_players
        .iter()
        .filter_map(|e| level.positions.get(e))
        .min_by_key(|p| p.coord.manhattan_distance(pos.coord))
}

pub fn find_path_to_adjacent(level: &Level, from: Coord, to: Coord) -> VecDeque<Coord> {
    let accept = |coord: Coord| level.map.tile(coord).walkable && level.unit_at(coord).is_none();
    let goal = |coord: Coord| coord.manhattan_distance(to) == 1;
    algorithm::breadth_first_search(from, accept, goal)
}
