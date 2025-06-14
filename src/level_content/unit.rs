use super::behavior;
use crate::engine::*;
use crate::level_content::behavior::find_nearest_visible_player;
use crate::level_model::*;
use std::collections::HashSet;

pub fn add_goon(level: &mut Level, coord: Coord) {
    let entity = level.next_id;
    level.next_id.0 += 1;
    level.turn_queue.push_back(entity);

    level.positions.insert(entity, Position::new(entity, coord));

    level.units.insert(
        entity,
        Unit::new(
            entity,
            ShortString::new("Goon"),
            Glyph::new('g', WHITE),
            Side::NPC,
            3, // vision
            2, // movement
            5, // hp_max
        ),
    );

    level.vision_memory.insert(
        entity,
        VisionMemory {
            entity,
            last_seen_player: None,
            visible_players: HashSet::new(),
        },
    );

    let select_action = |level: &Level| {
        let (_, _, pos, memory) = behavior::unpack_npc(level)?;
        let player = behavior::find_nearest_visible_player(level, pos, memory)?;
        behavior::basic_attack("bonk".to_string(), pos, player)
    };

    level.behaviors.insert(
        entity,
        Behavior {
            entity,
            select_move: behavior::standard_move,
            select_action,
        },
    );
}

pub fn add_hero(level: &mut Level, coord: Coord) {
    let entity = level.next_id;
    level.next_id.0 += 1;
    level.turn_queue.push_back(entity);

    level.positions.insert(entity, Position::new(entity, coord));

    level.units.insert(
        entity,
        Unit::new(
            entity,
            ShortString::new("Hero"),
            Glyph::new('@', WHITE),
            Side::Player,
            3,  // vision
            3,  // movement
            10, // hp_max
        ),
    );
}
