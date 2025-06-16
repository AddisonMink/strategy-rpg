use std::collections::VecDeque;
use std::mem;

use crate::engine::*;
use crate::level_model::*;

/// Standard move behavior.
/// If there is a visible player, it moves towards the nearest one.
/// If no visible player is found, it moves towards the last seen player position.
/// If no players are visible or last seen, it does not move.
pub fn standard_move(level: &Level) -> Option<VecDeque<Coord>> {
    let npc = level.active_unit()?;
    let player_opt = find_nearest_visible_player(level, npc);

    let mut path = if let Some(player) = player_opt {
        find_path_to_adjacent(level, npc.coord, player.coord)
    } else if let Some((_, player_pos)) = npc.memory.last_seen_player {
        find_path_to(level, npc.coord, player_pos)
    } else {
        None
    }?;

    path.truncate(npc.movement as usize);
    Some(path)
}

pub fn basic_attack(
    attack_name: String,
    min_damage: u16,
    max_damage: u16,
    attacker: &Unit,
    defender: &Unit,
) -> Option<VecDeque<Effect>> {
    (attacker.coord.manhattan_distance(defender.coord) == 1).then_some(())?;
    let direction = attacker.coord.direction_to(defender.coord)?;

    let mut effects = VecDeque::new();

    effects.push_back(Effect::Sleep { duration: 0.5 });

    effects.push_back(Effect::Animation {
        animation: Animation::panel_text(attacker.coord, attack_name.to_string().to_uppercase()),
    });

    effects.push_back(Effect::Animation {
        animation: Animation::attack(attacker.entity, direction),
    });

    effects.push_back(Effect::Damage {
        entity: defender.entity,
        min: min_damage,
        max: max_damage,
    });

    Some(effects)
}

pub fn find_nearest_visible_player<'a>(level: &'a Level, npc: &Unit) -> Option<&'a Unit> {
    npc.memory
        .visible_players
        .iter()
        .filter_map(|entity| level.units.get(entity))
        .min_by_key(|p| p.coord.manhattan_distance(npc.coord))
}

pub fn find_path_to(level: &Level, from: Coord, to: Coord) -> Option<VecDeque<Coord>> {
    let accept = |coord: Coord| level.map.tile(coord).walkable && level.unit_at(coord).is_none();
    let goal = |coord: Coord| coord == to;
    let path = algorithm::breadth_first_search(from, accept, goal);
    (!path.is_empty()).then_some(path)
}

pub fn find_path_to_adjacent(level: &Level, from: Coord, to: Coord) -> Option<VecDeque<Coord>> {
    let accept = |coord: Coord| level.map.tile(coord).walkable && level.unit_at(coord).is_none();
    let goal = |coord: Coord| coord.manhattan_distance(to) == 1;
    let path = algorithm::breadth_first_search(from, accept, goal);
    (!path.is_empty()).then_some(path)
}
