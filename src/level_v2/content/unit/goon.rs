use super::*;
use std::collections::VecDeque;

pub const GOON_DATA: UnitData = UnitData {
    name: ShortString::new("Goon"),
    glyph: Glyph::new('g', WHITE),
    side: Side::NPC,
    vision: 5,
    movement: 3,
    hp_max: 5,
    behavior: Some(UnitBehavior {
        select_move: select_move,
        select_action: select_action,
    }),
};

fn select_move(world: &World) -> Option<VecDeque<Coord>> {
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

fn select_action(world: &World) -> Option<VecDeque<Effect>> {
    let npc = world.active_unit()?;
    let nearest_player = nearest_player(world, npc)?;
    (nearest_player.coord.manhattan_distance(npc.coord) == 1).then_some(())?;

    let mut effects = VecDeque::new();
    let dir = npc.coord.direction_to(nearest_player.coord).unwrap();

    effects.push_back(Effect::Animate {
        animation: Animation::attack(npc.id(), dir),
    });

    effects.push_back(Effect::Damage {
        id: nearest_player.id(),
        min: 1,
        max: 3,
    });

    Some(effects)
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
