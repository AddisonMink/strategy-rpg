use super::*;
use std::collections::VecDeque;

pub const SHADOW_DATA: UnitData = UnitData {
    name: ShortString::new("SHADOW"),
    glyph: Glyph::new('S', WHITE),
    side: Side::NPC,
    vision: 99,
    movement: 3,
    hp_max: 3,
    behavior: Some(UnitBehavior {
        select_move: default_select_move,
        select_action: select_action,
    }),
};

fn select_action(world: &World) -> Option<VecDeque<Effect>> {
    let (mut effects, target_id) = begin_melee_attack(world)?;

    effects.push_back(Effect::Damage {
        id: target_id,
        min: 1,
        max: 3,
    });

    Some(effects)
}
