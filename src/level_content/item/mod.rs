use crate::engine::*;
use crate::level_model::*;

pub const SWORD: Item = Item {
    id: ItemId(1),
    name: ShortString::new("Sword"),
    color: LIGHTGRAY,
    uses_max: 5,
    uses: 5,
    actions: ShortList::new(&[SLASH]),
};

const SLASH: Action = Action {
    name: ShortString::new("Slash"),
    range: Range::SingleUnit { min: 1, max: 1 },
    cost: 1,
    effects: ShortList::new(&[
        EffectTemplate::AttackAnimation,
        EffectTemplate::Damage { min: 1, max: 4 },
    ]),
};
