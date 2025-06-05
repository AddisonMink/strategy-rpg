use std::vec;

use super::Action;
use crate::engine::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u16);

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    // Identifiers
    pub id: UnitId,
    pub name: ShortString,
    pub glyph: Glyph,
    // Attributes
    pub movement: u16,
    pub hp_max: u16,
    // State
    pub coord: Coord,
    pub hp: u16,
}

impl Unit {
    pub fn actions(&self) -> Vec<Action> {
        vec![Action::ATTACK]
    }
}
