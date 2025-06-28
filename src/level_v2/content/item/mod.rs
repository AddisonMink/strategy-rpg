use super::*;

pub const SWORD_DATA: ItemData = ItemData {
    id: ItemId(0),
    name: ShortString::new("Sword"),
    color: Color::new(0.8, 0.8, 0.8, 1.0),
    charges_max: 1,
    actions: ShortList::new(&[Action {
        name: ShortString::new("Slash"),
        range: ActionRange::Enemy {
            min_range: 1,
            max_range: 1,
        },
        effects: ShortList::new(&[
            ActionEffect::Attack,
            ActionEffect::Damage { min: 1, max: 3 },
        ]),
    }]),
};
