use super::*;
use std::collections::VecDeque;

pub const SHADOW_DATA: UnitData = UnitData {
    name: ShortString::new("SHADOW"),
    glyph: Glyph::new('S', WHITE),
    side: Side::NPC,
    vision: 99,
    movement: 3,
    magic: 0,
    strength: 0,
    hp_max: 3,
    tags: ShortList::new(&[UnitTag::Lurker]),
    behavior: Some(UnitBehavior {
        select_move: default_select_move,
        select_action: select_action,
    }),
};

fn select_action(world: &World) -> Option<VecDeque<Effect>> {
    let npc = world.active_unit()?;
    let in_darkness = world.light_grid.distance_from_light(npc.coord) > 0;

    let (name, min, max) = if in_darkness {
        (ShortString::new("Rend"), 2, 5)
    } else {
        (ShortString::new("Scratch"), 0, 2)
    };

    let (mut effects, target_id) = begin_melee_attack(world, name)?;

    effects.push_back(Effect::Damage {
        id: target_id,
        min,
        max,
    });

    Some(effects)
}
