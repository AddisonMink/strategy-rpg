use super::World;
use crate::util::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Player,
    NPC,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitData {
    pub name: ShortString,
    pub glyph: Glyph,
    pub side: Side,
    pub vision: u16,
    pub movement: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct UnitBehavior {
    pub select_move: fn(&World) -> Option<VecDeque<Coord>>,
}

impl Default for UnitBehavior {
    fn default() -> Self {
        Self {
            select_move: |_| None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    id: UnitId,
    data: UnitData,
    unit_behavior: UnitBehavior,
    pub coord: Coord,
}

impl Unit {
    pub fn new(id: UnitId, data: UnitData, coord: Coord) -> Self {
        Self {
            id,
            data,
            coord,
            unit_behavior: UnitBehavior::default(),
        }
    }

    pub fn new_with_behavior(
        id: UnitId,
        data: UnitData,
        coord: Coord,
        unit_behavior: UnitBehavior,
    ) -> Self {
        Self {
            id,
            data,
            coord,
            unit_behavior,
        }
    }

    pub fn id(&self) -> UnitId {
        self.id
    }

    pub fn data(&self) -> &UnitData {
        &self.data
    }

    pub fn behavior(&self) -> &UnitBehavior {
        &self.unit_behavior
    }
}
