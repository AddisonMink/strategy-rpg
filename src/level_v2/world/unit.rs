use crate::util::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct UnitData {
    pub name: ShortString,
    pub glyph: Glyph,
}

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    _id: UnitId,
    _data: UnitData,
    pub coord: Coord,
}

impl Unit {
    pub fn new(id: UnitId, data: UnitData, coord: Coord) -> Self {
        Self {
            _id: id,
            _data: data,
            coord,
        }
    }

    pub fn id(&self) -> UnitId {
        self._id
    }

    pub fn data(&self) -> &UnitData {
        &self._data
    }
}
