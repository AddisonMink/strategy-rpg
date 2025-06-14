use super::*;
use std::collections::VecDeque;

const NAME: ShortString = ShortString::new("Shadow");
const GLYPH: Glyph = Glyph::new('S', WHITE);
const VISION: u16 = 99;
const MOVEMENT: u16 = 3;
const HP_MAX: u16 = 3;
const TAGS: ShortList<EntityTag> = ShortList::new(&[EntityTag::Lurker]);

pub fn add_shadow(level: &mut Level, coord: Coord) -> Entity {
    let entity = level.next_id;
    level.next_id.0 += 1;
    level.turn_queue.push_back(entity);

    level.positions.insert(entity, Position::new(entity, coord));

    level.units.insert(
        entity,
        Unit::new(entity, NAME, GLYPH, Side::NPC, VISION, MOVEMENT, HP_MAX),
    );

    level
        .vision_memory
        .insert(entity, VisionMemory::new(entity));

    level.tags.insert(entity, Tags::new(entity, &TAGS));

    level.behaviors.insert(
        entity,
        Behavior {
            entity,
            select_move: behavior::standard_move,
            select_action,
        },
    );

    entity
}

fn select_action(level: &Level) -> Option<VecDeque<Effect>> {
    let (_, _, pos, memory) = behavior::unpack_npc(level)?;
    let player = behavior::find_nearest_visible_player(level, pos, memory)?;
    let in_darkness = level.light_grid.distance_from_light(pos.coord) > 0;

    let (attack, min_damage, max_damage) = if in_darkness {
        ("REND", 2, 5)
    } else {
        ("RAKE", 0, 2)
    };

    behavior::basic_attack(attack.to_string(), min_damage, max_damage, pos, player)
}
