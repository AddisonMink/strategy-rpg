use super::*;
use std::collections::VecDeque;

const NAME: ShortString = ShortString::new("Goon");
const GLYPH: Glyph = Glyph::new('g', WHITE);
const VISION: u16 = 3;
const MOVEMENT: u16 = 2;
const HP_MAX: u16 = 5;

pub fn add_goon(level: &mut Level, coord: Coord) {
    let entity = level.next_id;
    level.next_id.0 += 1;
    level.turn_queue.push_back(entity);

    level.positions.insert(entity, Position::new(entity, coord));

    level.units.insert(
        entity,
        Unit::new(entity, NAME, GLYPH, Side::NPC, VISION, MOVEMENT, HP_MAX),
    );

    level.vision_memory.insert(
        entity,
        VisionMemory {
            entity,
            last_seen_player: None,
            visible_players: HashSet::new(),
        },
    );

    level.behaviors.insert(
        entity,
        Behavior {
            entity,
            select_move: behavior::standard_move,
            select_action,
        },
    );
}

fn select_action(level: &Level) -> Option<VecDeque<Effect>> {
    let (_, _, pos, memory) = behavior::unpack_npc(level)?;
    let player = behavior::find_nearest_visible_player(level, pos, memory)?;
    behavior::basic_attack("bonk".to_string(), pos, player)
}
