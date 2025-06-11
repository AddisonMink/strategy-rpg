use crate::engine::*;
use crate::level_model::*;

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
            5,  // vision
            3,  // movement
            10, // hp_max
        ),
    );
}
