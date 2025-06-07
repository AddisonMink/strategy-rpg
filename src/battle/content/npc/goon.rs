use std::collections::VecDeque;

use super::*;

const DATA: UnitData = UnitData {
    name: ShortString::new("Goon"),
    glyph: Glyph::new('G', WHITE),
    side: Side::NPC,
    movement: 2,
    hp_max: 5,
};

pub fn make_goon(id: UnitId, coord: Coord) -> Unit {
    Unit::new_npc(
        id,
        coord,
        DATA,
        behavior::chase_nearest_player,
        select_action,
    )
}

fn select_action(battle: &Battle, unit: &Unit) -> Option<VecDeque<Effect>> {
    let player_id = behavior::nearest_player(battle, unit)?;
    let player = battle.unit(player_id)?;
    (player.coord.manhattan_distance(unit.coord) <= 1).then_some(())?;
    let direction = unit.coord.direction_to(player.coord)?;

    let effects = VecDeque::from_iter([
        Effect::QueueAnimation {
            animation: Animation::attack(unit.id, direction),
        },
        Effect::Damage {
            min: 0,
            max: 3,
            target: player_id,
        },
    ]);
    Some(effects)
}
