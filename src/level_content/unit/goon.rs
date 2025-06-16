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

    let behavior = Behavior {
        select_move: behavior::standard_move,
        select_action,
    };

    level.units.insert(
        entity,
        Unit::new(
            entity,
            NAME,
            GLYPH,
            Side::NPC,
            VISION,
            MOVEMENT,
            HP_MAX,
            coord,
            Some(behavior.clone()),
        ),
    );
}

fn select_action(level: &Level) -> Option<VecDeque<Effect>> {
    let npc = level.active_unit()?;
    let player = behavior::find_nearest_visible_player(level, npc)?;
    behavior::basic_attack("bonk".to_string(), 1, 3, npc, player)
}
