use super::light::Light;
use crate::util::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    // identifiers
    pub id: UnitId,
    pub glyph: Glyph,

    // attributes
    pub vision: u16,

    // state
    pub coord: Coord,
    pub light: Option<Light>,
}
