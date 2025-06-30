use super::*;

pub const MAGICIAN_DATA: UnitData = UnitData {
    name: ShortString::new("Magician"),
    glyph: Glyph::new('@', WHITE),
    side: Side::Player,
    vision: 2,
    movement: 3,
    strength: 0,
    magic: 1,
    hp_max: 5,
    tags: ShortList::empty(),
    behavior: None,
};
