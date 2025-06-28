use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct ItemData {
    pub id: ItemId,
    pub name: ShortString,
    pub color: Color,
    pub charges_max: u16,
    pub actions: ShortList<Action>,
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    data: ItemData,
    pub charges: u16,
}

impl Item {
    pub fn new(data: ItemData) -> Self {
        Self {
            data,
            charges: data.charges_max,
        }
    }

    pub fn data(&self) -> &ItemData {
        &self.data
    }

    pub fn actions(&self) -> Vec<ItemAction> {
        self.data
            .actions
            .iter()
            .map(|action| ItemAction {
                item_name: self.data.name,
                item_id: self.data.id,
                item_color: self.data.color,
                action: *action,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemAction {
    pub item_name: ShortString,
    pub item_id: ItemId,
    pub item_color: Color,
    pub action: Action,
}
