use std::collections::VecDeque;

use super::*;

const DATA: UnitData = UnitData {
    name: ShortString::new("Goon"),
    glyph: Glyph::new('G', WHITE),
    side: Side::NPC,
    movement: 2,
    vision: 2,
    hp_max: 5,
};

pub fn make_goon(id: UnitId, coord: Coord) -> Unit {
    Unit::new_npc(id, coord, DATA, select_move, select_action)
}

fn select_move(battle: &Battle, unit: &Unit) -> Option<VecDeque<Coord>> {
    let player = behavior::find_nearest_visible_player(battle, unit.id)?;
    let path = behavior::find_path_to_adjacent(battle, unit, player.coord);
    if path.is_empty() { None } else { Some(path) }
}

fn select_action(battle: &Battle, unit: &Unit) -> Option<VecDeque<Effect>> {
    let player = behavior::find_nearest_visible_player(battle, unit.id)?;
    (player.coord.manhattan_distance(unit.coord) <= 1).then_some(())?;
    let direction = unit.coord.direction_to(player.coord)?;

    let effects = VecDeque::from_iter([
        Effect::QueueAnimation {
            animation: Animation::action_message(unit, ShortString::new("Bonk"), RED),
        },
        Effect::QueueAnimation {
            animation: Animation::attack(unit.id, direction),
        },
        Effect::Damage {
            min: 0,
            max: 3,
            target: player.id,
        },
    ]);
    Some(effects)
}
