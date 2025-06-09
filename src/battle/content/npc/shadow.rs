use super::*;
use std::collections::VecDeque;

const DATA: UnitData = UnitData {
    name: ShortString::new("Shadow"),
    glyph: Glyph::new('S', WHITE),
    side: Side::NPC,
    movement: 3,
    vision: 99,
    hp_max: 3,
};

pub fn make_shadow(id: UnitId, coord: Coord) -> Unit {
    Unit::new_npc(id, coord, DATA, behavior::standard_move, select_action)
}

fn select_action(battle: &Battle, unit: &Unit) -> Option<VecDeque<Effect>> {
    let player = behavior::find_nearest_visible_player(battle, unit.id)?;
    (player.coord.manhattan_distance(unit.coord) <= 1).then_some(())?;
    let direction = unit.coord.direction_to(player.coord)?;

    let effects = VecDeque::from_iter([
        Effect::QueueAnimation {
            animation: Animation::action_message(unit, ShortString::new("Consume"), RED),
        },
        Effect::QueueAnimation {
            animation: Animation::attack(unit.id, direction),
        },
        Effect::Damage {
            min: 1,
            max: 5,
            target: player.id,
        },
    ]);
    Some(effects)
}
