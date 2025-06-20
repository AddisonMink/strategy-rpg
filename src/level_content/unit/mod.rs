mod behavior;
mod goon;
mod shadow;

use crate::engine::*;
use crate::level_content::item;
use crate::level_model::*;
use std::collections::HashSet;

pub use goon::add_goon;
pub use shadow::add_shadow;

pub fn add_hero(level: &mut Level, coord: Coord) {
    let entity = level.next_unit_id;
    level.next_unit_id.0 += 1;
    level.turn_queue.push_back(entity);

    level.units.insert(
        entity,
        Unit::new(
            entity,
            ShortString::new("Hero"),
            Glyph::new('@', WHITE),
            Side::Player,
            3,  // vision
            3,  // movement
            10, // hp_max,
            coord,
            None,
        ),
    );

    let hero = level.units.get_mut(&entity).unwrap();
    hero.add_item(item::SWORD);
    hero.add_item(item::TORCH);
}
