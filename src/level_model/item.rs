use crate::engine::*;
use crate::level_model::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u32);

pub struct Item {
    pub id: ItemId,
    pub name: ShortString,
    pub color: Color,
    pub actions: ShortList<Action>,
}
