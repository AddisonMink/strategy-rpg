mod goon;
mod shadow;

use std::collections::VecDeque;

use super::*;

pub use goon::GOON_DATA;
pub use shadow::SHADOW_DATA;

fn default_select_move(world: &World) -> Option<VecDeque<Coord>> {
    let npc = world.active_unit()?;

    if let Some(nearest_player) = nearest_player(world, npc) {
        let path = find_path_to_adjacent(world, npc, nearest_player.coord);
        (!path.is_empty()).then_some(path)
    } else if let Some((_, coord)) = npc.memory.last_seen_player {
        let path = find_path_to(world, npc, coord);
        (!path.is_empty()).then_some(path)
    } else {
        None
    }
}

fn begin_melee_attack(world: &World) -> Option<(VecDeque<Effect>, UnitId)> {
    let npc = world.active_unit()?;
    let nearest_player = nearest_player(world, npc)?;
    let dir = npc.coord.direction_to(nearest_player.coord).unwrap();
    (nearest_player.coord.manhattan_distance(npc.coord) == 1).then_some(())?;

    let effects = VecDeque::from([Effect::Animate {
        animation: Animation::attack(npc.id(), dir),
    }]);

    Some((effects, nearest_player.id()))
}

fn nearest_player<'a>(world: &'a World, npc: &Unit) -> Option<&'a Unit> {
    npc.memory
        .visible_players
        .iter()
        .filter_map(|&id| world.unit(id))
        .min_by_key(|player| player.coord.manhattan_distance(npc.coord))
}

fn find_path_to(world: &World, npc: &Unit, end: Coord) -> VecDeque<Coord> {
    let accept = |coord: Coord| world.valid_move(coord);
    let goal = |coord: Coord| coord == end;
    let mut path = breadth_first_search(npc.coord, accept, goal);
    path.truncate(npc.data().movement as usize);
    path
}

fn find_path_to_adjacent(world: &World, npc: &Unit, end: Coord) -> VecDeque<Coord> {
    let accept = |coord: Coord| world.valid_move(coord);
    let goal = |coord: Coord| coord.manhattan_distance(end) == 1;
    let mut path = breadth_first_search(npc.coord, accept, goal);
    path.truncate(npc.data().movement as usize);
    path
}
