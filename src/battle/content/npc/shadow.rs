use super::*;
use std::collections::VecDeque;

const DATA: UnitData = UnitData {
    name: ShortString::new("Shadow"),
    glyph: Glyph::new('S', WHITE),
    side: Side::NPC,
    movement: 3,
    vision: 99,
    hp_max: 3,
    tags: ShortList::new(&[UnitTag::Lurker]),
};

pub fn make_shadow(id: UnitId, coord: Coord) -> Unit {
    Unit::new(
        id,
        coord,
        DATA,
        Some(behavior::standard_move),
        Some(select_action),
    )
}

fn select_action(battle: &Battle) -> VecDeque<Effect> {
    let mut effects: VecDeque<Effect> = VecDeque::new();
    let unit = battle.active_unit().expect("Active unit should exist");

    let Some(player) = behavior::find_nearest_visible_player(battle, unit.id) else {
        return effects;
    };

    if player.coord.manhattan_distance(unit.coord) > 1 {
        return effects;
    }

    let direction = unit.coord.direction_to(player.coord).unwrap();

    effects.extend([
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

    effects
}
