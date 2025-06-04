use crate::engine::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub id: UnitId,
    pub name: ShortString,
    pub glyph: Glyph,
    pub coord: Coord,
}
