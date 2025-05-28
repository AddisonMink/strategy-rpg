use super::light::Light;
use crate::util::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone)]
pub struct Unit {
    // identifiers
    pub id: UnitId,
    pub glyph: Glyph,
    pub name: String,

    // attributes
    pub vision: u16,
    pub movement: u16,

    // state
    pub coord: Coord,
    pub light: Option<Light>,
}
