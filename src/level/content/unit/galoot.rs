use super::*;

pub const GALOOT_DATA: UnitData = UnitData {
    name: ShortString::new("Galoot"),
    glyph: Glyph::new('@', WHITE),
    side: Side::Player,
    vision: 2,
    movement: 3,
    strength: 1,
    magic: 0,
    hp_max: 5,
    tags: ShortList::empty(),
    behavior: None,
};
